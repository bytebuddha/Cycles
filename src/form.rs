use gtk::prelude::*;
use relm::{connect, Component, Relm, Widget, Update};
use relm_derive::Msg;
use chrono::Datelike;

use crate::description::{Description, Interval};

#[derive(Msg)]
pub enum Msg {
    Created(Description),
    CreateClicked,
    ColorSet(gdk::RGBA),
    TagsChanged(Vec<String>)
}

pub struct Widgets {
    container: gtk::Popover,
    create_btn: gtk::Button,
    label_entry: gtk::Entry,
    description_entry: gtk::Entry,
    cycle_interval: gtk::ComboBox,
    interval_multiplier: gtk::SpinButton,
    datepicker: gtk::Calendar,
    color_entry: gtk::ColorButton,
}

pub struct Form {
    model: Model,
    widgets: Widgets,
}

pub struct Model {
    relm: Relm<Form>,
    btn: gtk::Button,
    description: Option<Description>,
    color: Option<String>,
    tag_data: Vec<String>,
    tags: Option<Component<crate::tags::Tags>>
}

impl Update for Form {
    type Model = Model;
    type ModelParam = (gtk::Button, Option<Description>);
    type Msg = Msg;

    fn model(relm: &Relm<Self>, (btn, description): (gtk::Button, Option<Description>)) -> Model {
        Model {
            relm: relm.clone(),
            btn, description,
            color: None,
            tags: None,
            tag_data: vec![]
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::TagsChanged(tags) => {
                self.model.tag_data = tags;
            },
            Msg::Created(_) => {
                self.widgets.container.hide();
            },
            Msg::ColorSet(color) => {
                self.model.color = Some(color.to_string());
            },
            Msg::CreateClicked => {
                let label: String = self.widgets.label_entry.get_text().into();
                let _description: String = self.widgets.description_entry.get_text().into();
                let description = if !_description.is_empty() { Some(_description) } else { None };
                let interval = match self.widgets.cycle_interval.get_active() {
                    Some(0) => Interval::Weekly,
                    Some(1) => Interval::Monthly,
                    Some(2) => Interval::Yearly,
                    _ => unreachable!()
                };
                let interval_multiplier = self.widgets.interval_multiplier.get_value_as_int() as i64;
                let (y, m, d) = self.widgets.datepicker.get_date();
                let start = chrono::NaiveDate::from_ymd(y as i32, m + 1, d);
                let description = Description {
                    tags: self.model.tag_data.clone(),
                    label, interval,
                    description,
                    interval_multiplier, start,
                    color: self.model.color.clone()
                };
                self.model.relm.stream().emit(Msg::Created(description));
            }
        }
    }
}

impl Widget for Form {
    type Root = gtk::Popover;

    fn root(&self) -> Self::Root {
        self.widgets.container.clone()
    }

    fn view(_relm: &Relm<Self>, mut model: Self::Model) -> Self {
        let glade_src = include_str!("../ui/form.ui");
        let builder = gtk::Builder::from_string(glade_src);
        let container: gtk::Popover = or_panic!(builder.get_object("createPopover"));
        let create_container: gtk::Box = or_panic!(builder.get_object("createContainer"));
        let create_btn: gtk::Button = or_panic!(builder.get_object("createButton"));
        let label_entry: gtk::Entry = or_panic!(builder.get_object("labelEntry"));
        let description_entry: gtk::Entry = or_panic!(builder.get_object("descriptionEntry"));
        let color_entry: gtk::ColorButton = or_panic!(builder.get_object("colorEntry"));
        let cycle_interval: gtk::ComboBox = or_panic!(builder.get_object("intervalSelect"));
        let interval_multiplier: gtk::SpinButton = or_panic!(builder.get_object("intervalMultiplier"));
        let datepicker: gtk::Calendar = or_panic!(builder.get_object("startDate"));

        let tags_entry = relm::init::<crate::tags::Tags>(vec![]).expect("Failed to create tags_entry");
        let stream = _relm.stream().clone();
        tags_entry.stream().observe(move |msg| {
            if let crate::tags::Msg::Changed(a) = msg {
                stream.emit(Msg::TagsChanged(a.clone()));
            }
        });
        create_container.add(tags_entry.widget());

        connect!(_relm, color_entry, connect_color_set(x), Msg::ColorSet(x.get_rgba()));
        connect!(_relm, create_btn, connect_clicked(_), Msg::CreateClicked);

        if let Some(desc) = model.description.as_ref() {
            label_entry.set_text(&desc.label);
            tags_entry.stream().emit(crate::tags::Msg::SetTags(desc.tags.clone()));
            if let Some(description) = &desc.description {
                description_entry.set_text(&description);
            }
            interval_multiplier.set_value(desc.interval_multiplier as f64);
            datepicker.set_property_day(desc.start.day() as i32);
            datepicker.set_property_month(desc.start.month() as i32 - 1);
            datepicker.set_property_year(desc.start.year() as i32);
            if let Some(color) = desc.color.as_ref() {
                use std::str::FromStr;
                color_entry.set_rgba(&or_panic!(Res gdk::RGBA::from_str(color)))
            }

            match desc.interval {
                Interval::Weekly => cycle_interval.set_active(Some(0)),
                Interval::Monthly => cycle_interval.set_active(Some(1)),
                Interval::Yearly => cycle_interval.set_active(Some(2))
            }
        }

        model.tags = Some(tags_entry);
        container.set_relative_to(Some(&model.btn));
        container.show_all();

        Form {
            model,
            widgets: Widgets {
                container,
                create_btn,
                description_entry,
                label_entry,
                cycle_interval,
                interval_multiplier,
                datepicker,
                color_entry,
            },
        }
    }
}
