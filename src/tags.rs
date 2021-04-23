use gtk::prelude::*;
use relm::{connect, Component, Relm, Widget, Update};
use relm_derive::Msg;
use std::collections::HashMap;

#[derive(Msg)]
pub enum Msg {
    Add,
    Delete(usize),
    Updated((usize, String)),
    Changed(Vec<String>),
    SetTags(Vec<String>)
}

pub struct Tags {
    model: Model,
    container: gtk::Box,
    expander: gtk::Expander,
    tags_list: gtk::Box,
    create_button: gtk::Button
}

pub struct Model {
    tags: Vec<String>,
    relm: Relm<Tags>,
    entries: HashMap<usize, (String, Component<crate::tag::Tag>)>
}

impl Update for Tags {
    type Model = Model;
    type ModelParam = Vec<String>;
    type Msg = Msg;

    fn model(relm: &Relm<Self>, tags: Vec<String>) -> Model {
        Model {
            tags,
            relm: relm.clone(),
            entries: Default::default()
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SetTags(tags) => {
                for tag in tags {
                    let id = self.model.entries.len();
                    let element = relm::init::<crate::tag::Tag>(tag.clone()).expect("Failed to create tag widget");
                    let stream = self.model.relm.stream().clone();
                    element.stream().observe(move |msg| {
                        match msg {
                            crate::tag::Msg::Update(a) => {
                                stream.emit(Msg::Updated((id, a.clone())))
                            },
                            crate::tag::Msg::Delete => {
                                stream.emit(Msg::Delete(id));
                            },
                        }
                    });
                    self.tags_list.add(element.widget());
                    self.model.entries.insert(id, (tag, element));
                }
            },
            Msg::Add => {
                let id = self.model.entries.len();
                let element = relm::init::<crate::tag::Tag>(String::new()).expect("Failed to create tag widget");
                let stream = self.model.relm.stream().clone();
                element.stream().observe(move |msg| {
                    match msg {
                        crate::tag::Msg::Update(a) => {
                            stream.emit(Msg::Updated((id, a.clone())))
                        },
                        crate::tag::Msg::Delete => {
                            stream.emit(Msg::Delete(id));
                        },
                    }
                });
                self.tags_list.add(element.widget());
                self.model.entries.insert(id, (String::new(), element));
            },
            Msg::Delete(item) => {
                let (_, element) = or_panic!(self.model.entries.remove(&item));
                self.tags_list.remove(element.widget());
                self.model.relm.stream().emit(Msg::Changed(self.model.entries.values().map(|(data, _)| data.clone()).collect()));
            },
            Msg::Updated((id, string)) => {
                self.model.entries.get_mut(&id).map(|(x, _)| *x = string);
                self.model.relm.stream().emit(Msg::Changed(self.model.entries.values().map(|(data, _)| data.clone()).collect()));
            },
            _ => {}
        }
    }
}

impl Widget for Tags {
    type Root = gtk::Expander;

    fn root(&self) -> Self::Root {
        self.expander.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("../ui/tags.ui");
        let builder = gtk::Builder::from_string(glade_src);
        let expander: gtk::Expander = or_panic!(builder.get_object("tagsExpander"));
        let container: gtk::Box = or_panic!(builder.get_object("tagsContainer"));
        let tags_list: gtk::Box = or_panic!(builder.get_object("tagsList"));
        let create_button: gtk::Button = or_panic!(builder.get_object("createButton"));

        connect!(_relm, create_button, connect_clicked(_), Msg::Add);

        container.show_all();

        Tags {
            model,
            container,
            expander,
            tags_list,
            create_button
        }
    }
}
