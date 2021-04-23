use uuid::Uuid;
use gtk::prelude::*;
use relm::{connect, init, Component, Relm, Widget, Update};
use relm_derive::Msg;

use crate::badge::Badge;
use crate::form::Form;
use crate::description::{Description, Interval};

fn level_styles(color: &str) -> String {
    format!(
        "levelbar block.filled {{background-color: {color};border-color:{color};}}",
        color=color
    )
}

#[derive(Msg)]
pub enum Msg {
    Updated((Uuid, Description)),
    Delete(Uuid),
    ConfirmDelete,
    CancelDelete,
    OpenEdit
}

pub struct Widgets {
    container: gtk::Box,
    tags_container: gtk::FlowBox,
    label: gtk::Label,
    description: gtk::Label,
    start_end: gtk::Label,
    days_remaining: gtk::Label,
    cycle_level: gtk::LevelBar,
    edit_btn: gtk::Button,
    delete_btn: gtk::Button,
    delete_popover: gtk::Popover,
    no_btn: gtk::Button,
    yes_btn: gtk::Button,
}

pub struct Cycle {
    model: Model,
    widgets: Widgets
}

pub struct Model {
    id: Uuid,
    relm: Relm<Cycle>,
    tags: Vec<(String, Component<Badge>)>,
    description: Description,
    edit_popover: Option<Component<Form>>
}

impl Update for Cycle {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = (Uuid, Description);
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(relm: &Relm<Self>, (id, description): (Uuid, Description)) -> Model {
        Model {
            id,
            tags: vec![],
            relm: relm.clone(),
            description,
            edit_popover: None
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Delete(_) => {
                self.widgets.delete_popover.hide();
            },
            Msg::ConfirmDelete => {
                self.widgets.delete_popover.show();
            },
            Msg::CancelDelete => {
                self.widgets.delete_popover.hide();
            },
            Msg::OpenEdit => {
                let desc = Some(self.model.description.clone());
                let elem = self.widgets.edit_btn.clone();
                let element = or_panic!(Res init::<Form>((elem, desc)));
                let stream = self.model.relm.stream().clone();
                let id = self.model.id;
                element.stream().observe(move |msg| {
                    if let crate::form::Msg::Created(a) = msg {
                        stream.emit(Msg::Updated((id, a.clone())));
                    }
                });
                self.model.edit_popover = Some(element);
            },
            Msg::Updated((_, description)) => {
                if let Some(color) = description.color.as_ref() {
                    add_stylesheet!(raw level_styles(color).as_ref(), self.widgets.cycle_level);
                }
                for (_, element) in &self.model.tags {
                    self.widgets.tags_container.remove(element.widget());
                }
                self.model.tags = vec![];
                for tag in &description.tags {
                    let element = or_panic!(Res init::<crate::badge::Badge>(tag.clone()));
                    self.widgets.tags_container.add(element.widget());
                    self.model.tags.push((tag.clone(), element));
                }
                self.widgets.tags_container.show_all();
                self.widgets.label.set_text(&description.label);
                if let Some(desc) = &description.description {
                    self.widgets.description.set_text(&desc);
                }
                let args = self.model.description.render_arguments(None);
                self.widgets.start_end.set_text(&format!("{} - {}",
                    args.start_date.format("%m/%d"),
                    args.end_date.format("%m/%d")
                ));
                self.widgets.days_remaining.set_text(&format!("{}", args.remaining_days));
                self.widgets.cycle_level.set_min_value(0.);
                self.widgets.cycle_level.set_value(args.bar_value);
                self.widgets.cycle_level.set_max_value(args.bar_length);
                self.model.description = description;
            }
        }
    }
}

impl Widget for Cycle {
    // Specify the type of the root widget.
    type Root = gtk::Box;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.container.clone()
    }

    fn view(_relm: &Relm<Self>, mut model: Self::Model) -> Self {
        let glade_src = include_str!("../ui/cycle.ui");
        let builder = gtk::Builder::from_string(glade_src);
        let container: gtk::Box = or_panic!(builder.get_object("cycleContainer"));
        let label: gtk::Label = or_panic!(builder.get_object("cycleLabel"));
        let description: gtk::Label = or_panic!(builder.get_object("cycleDescription"));
        let start_end: gtk::Label = or_panic!(builder.get_object("cycleStartEnd"));
        let days_remaining: gtk::Label = or_panic!(builder.get_object("cycleDaysRemainingValue"));
        let cycle_level: gtk::LevelBar = or_panic!(builder.get_object("cycleLevel"));
        let delete_popover: gtk::Popover = or_panic!(builder.get_object("deletePopover"));
        let edit_btn: gtk::Button = or_panic!(builder.get_object("editButton"));
        let delete_btn: gtk::Button = or_panic!(builder.get_object("deleteButton"));
        let no_btn: gtk::Button = or_panic!(builder.get_object("deleteNoButton"));
        let yes_btn: gtk::Button = or_panic!(builder.get_object("deleteYesButton"));
        let tags_container: gtk::FlowBox = or_panic!(builder.get_object("tagsContainer"));

        let id = model.id;

        connect!(_relm, delete_btn, connect_clicked(_), Msg::ConfirmDelete);
        connect!(_relm, edit_btn, connect_clicked(_), Msg::OpenEdit);
        connect!(_relm, no_btn, connect_clicked(_), Msg::CancelDelete);
        connect!(_relm, yes_btn, connect_clicked(_), Msg::Delete(id));

        if let Some(color) = model.description.color.as_ref() {
            add_stylesheet!(raw level_styles(color).as_ref(), cycle_level);
        }

        for tag in &model.description.tags {
            let element = or_panic!(Res init::<crate::badge::Badge>(tag.clone()));
            tags_container.add(element.widget());
            model.tags.push((tag.clone(), element));
        }
        add_stylesheet!("../css/cycle.css", container);

        if let Some(desc) = model.description.description.as_ref() {
            description.set_text(&desc);
        }
        label.set_text(model.description.label.as_ref());
        let args = model.description.render_arguments(None);
        start_end.set_text(&format!("{} - {}",
            args.start_date.format("%m/%d"),
            args.end_date.format("%m/%d")
        ));
        days_remaining.set_text(&format!("{}", args.remaining_days));
        cycle_level.set_min_value(0.);
        cycle_level.set_value(args.bar_value);
        cycle_level.set_max_value(args.bar_length);
        container.show_all();

        Cycle {
            model,
            widgets: Widgets {
                container,
                tags_container,
                label,
                description,
                start_end,
                days_remaining,
                cycle_level,
                edit_btn,
                delete_btn,
                delete_popover,
                no_btn,
                yes_btn,
            },
        }
    }
}
