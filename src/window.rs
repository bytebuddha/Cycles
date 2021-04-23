#![allow(non_snake_case, dead_code, unused_imports)]
use uuid::Uuid;
use gtk::prelude::*;
use relm_derive::Msg;
use relm::{connect, init, Relm, Update, Widget, Component};

use std::path::PathBuf;
use std::collections::HashMap;

use crate::cycle::{Cycle, Msg as CycleMsg};
use crate::form::{Form, Msg as FormMsg};
use crate::menu::{Menu, Msg as MenuMsg};
use crate::about::About;
use crate::cache::Database;
use crate::description::{Description, Interval};

pub struct Model {
    relm: Relm<Win>,
    cycles: HashMap<Uuid, (Description, Component<Cycle>)>,
    descriptions: Database,
    create_popup: Option<Component<Form>>,
    menu_popup: Option<Component<Menu>>
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    About,
    DisplaySearch,
    RequestExport,
    Export(PathBuf),
    RequestImport,
    Import(PathBuf),
    OpenMenu,
    OpenCreate,
    Delete(Uuid),
    Query(String),
    Updated((Uuid, Description)),
    Add(Description)
}

#[derive(Clone)]
pub struct Widgets {
    window: libhandy::ApplicationWindow,
    menu_btn: gtk::Button,
    create_btn: gtk::Button,
    cycle_list: gtk::ListBox,
    search_entry: gtk::SearchEntry,
    search_button: gtk::Button,
    search_bar: libhandy::SearchBar
}

pub struct Win {
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            menu_popup: None,
            create_popup: None,
            cycles: Default::default(),
            descriptions: crate::cache::Database::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::DisplaySearch => {
                use libhandy::SearchBarExt;
                self.widgets.search_bar.set_search_mode(true);
            },
            Msg::Import(filename) => {
                let file = or_panic!(Res std::fs::File::open(filename));
                let descriptions = self.model.descriptions.import_from_reader(file);
                for (key, value) in descriptions {
                    self.append_description(key, value);
                }
            },
            Msg::Export(file) => {
                let file = or_panic!(Res std::fs::File::create(file));
                self.model.descriptions.export_to_writer(file);
            },
            Msg::RequestImport => {
                let dialog = gtk::FileChooserDialog::new(Some("Open a file"), Some(&self.widgets.window), gtk::FileChooserAction::Open);
                dialog.add_button("Cancel", gtk::ResponseType::Cancel);
                dialog.add_button("Accept", gtk::ResponseType::Accept);
                let result = dialog.run();
                if result == gtk::ResponseType::Accept {
                    if let Some(filename) = dialog.get_filename() {
                        self.model.relm.stream().emit(Msg::Import(filename));
                    }
                }
                dialog.close();
            },
            Msg::RequestExport => {
                let dialog = gtk::FileChooserDialog::new(Some("Open a file"), Some(&self.widgets.window), gtk::FileChooserAction::Save);
                dialog.add_button("Cancel", gtk::ResponseType::Cancel);
                dialog.add_button("Accept", gtk::ResponseType::Accept);
                let result = dialog.run();
                if result == gtk::ResponseType::Accept {
                    if let Some(filename) = dialog.get_filename() {
                        self.model.relm.stream().emit(Msg::Export(filename));
                    }
                }
                dialog.close();
            },
            Msg::Query(query) => {
                if query.is_empty() {
                    for (_, item) in self.model.cycles.values() {
                        item.widget().show()
                    }
                } else {
                    for (_, (value, widget)) in self.model.cycles.iter() {
                        let query = query.to_lowercase();
                        if value.label.to_lowercase().contains(&query) ||
                           value.description.is_some() &&
                           value.description.as_ref().unwrap().to_lowercase().contains(&query) ||
                           tags_contain_value(&value.tags, &query)
                        {
                             widget.widget().show();
                        } else {
                            widget.widget().hide();
                        }
                    }
                }
            },
            Msg::Quit => gtk::main_quit(),
            #[allow(unused_must_use)]
            Msg::About => {
                or_panic!(Res init::<About>(self.widgets.window.clone()));
            },
            Msg::OpenMenu => {
                let element = or_panic!(Res init::<Menu>(self.widgets.menu_btn.clone()));
                connect!(element@MenuMsg::About, self.model.relm, Msg::About);
                connect!(element@MenuMsg::Quit, self.model.relm, Msg::Quit);
                connect!(element@MenuMsg::Import, self.model.relm, Msg::RequestImport);
                connect!(element@MenuMsg::Export, self.model.relm, Msg::RequestExport);
                self.model.menu_popup = Some(element);
            },
            Msg::OpenCreate => {
                let element = or_panic!(Res init::<Form>((self.widgets.create_btn.clone(), None)));
                let stream = self.model.relm.stream().clone();
                element.stream().observe(move |msg| {
                    if let FormMsg::Created(a) = msg {
                        stream.emit(Msg::Add(a.clone()));
                    }
                });
                self.model.create_popup = Some(element);
            },
            Msg::Add(description) => {
                let id = self.model.descriptions.append(description.clone());
                self.append_description(id, description);
            },
            Msg::Delete(id) => {
                self.model.descriptions.remove(id);
                or_panic!(self.model.cycles.remove(&id)).1.widget().hide();
            },
            Msg::Updated((id, description)) => {
                self.model.descriptions.update(id, description);
                self.widgets.window.show_all();
            }
        }
    }
}

impl Widget for Win {
    type Root = libhandy::ApplicationWindow;
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("../ui/main.ui");
        let builder = gtk::Builder::from_string(glade_src);

        let window: libhandy::ApplicationWindow = or_panic!(builder.get_object("applicationWindow"));
        let menu_btn: gtk::Button = or_panic!(builder.get_object("menuButton"));
        let create_btn: gtk::Button = or_panic!(builder.get_object("createButton"));
        let cycle_list: gtk::ListBox = or_panic!(builder.get_object("cycleList"));
        let search_entry: gtk::SearchEntry = or_panic!(builder.get_object("searchEntry"));
        let search_button: gtk::Button = or_panic!(builder.get_object("searchButton"));
        let search_bar: libhandy::SearchBar = or_panic!(builder.get_object("searchBar"));

        connect!(relm, search_button, connect_clicked(_), Msg::DisplaySearch);
        connect!(relm, search_entry, connect_changed(btn), Msg::Query(btn.get_text().into()));
        connect!(relm, menu_btn, connect_clicked(_), Msg::OpenMenu);
        connect!(relm, create_btn, connect_clicked(_), Msg::OpenCreate);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        window.show_all();

        let mut window = Win {
            model,
            widgets: Widgets {
                window,
                search_bar,
                search_entry,
                menu_btn,
                create_btn,
                cycle_list,
                search_button
            },
        };
        window.load_descriptions();
        window
    }
}

impl Win {

    fn load_descriptions(&mut self) {
        for (id, cycle) in self.model.descriptions.entries() {
            self.append_description(id, cycle);
        }
    }

    fn append_description(&mut self, id: Uuid, description: Description) {
        self.model.cycles.insert(id, (description.clone(), self.description_element(id, description)));
    }

    fn description_element(&self, id: Uuid, description: Description) -> Component<Cycle> {
        let element = or_panic!(Res init::<Cycle>((id, description)));
        self.widgets.cycle_list.add(element.widget());
        let stream = self.model.relm.stream().clone();
        element.stream().observe(move |msg| {
            match msg {
                CycleMsg::Delete(a) => stream.emit(Msg::Delete(*a)),
                CycleMsg::Updated((id, data)) => stream.emit(Msg::Updated((*id, data.clone()))),
                _ => {}
            }
        });
        element
    }
}

fn tags_contain_value(tags: &Vec<String>, value: &str) -> bool {
    for tag in tags {
        if tag.to_lowercase().contains(&value.to_lowercase()) { return true }
    }
    false
}
