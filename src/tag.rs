use gtk::prelude::*;
use relm::{connect, Relm, Widget, Update};
use relm_derive::{Msg, widget};

#[derive(Debug, Msg)]
pub enum Msg {
    Delete,
    Update(String)
}

pub struct Model {
    relm: Relm<Tag>,
    text: String
}

#[widget]
impl Widget for Tag {
    fn init_view(&mut self) {}
    fn model(relm: &Relm<Self>, text: String) -> Model {
        Model {
            relm: relm.clone(),
            text
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Delete => {},
            Msg::Update(_) => {}
        }
    }

    view! {
        gtk::Box {
            #[name="entry"]
            gtk::Entry {
                margin_top: 5,
                margin_bottom: 5,
                margin_start: 5,
                text: &self.model.text,
                changed(entry) => Msg::Update(entry.get_text().into())
            },
            #[name="delete_btn"]
            gtk::Button {
                margin_top: 5,
                margin_bottom: 5,
                margin_end: 5,
                clicked() => Msg::Delete,
                gtk::Image {
                    property_icon_name: Some("user-trash-symbolic")
                }
            }
        }
    }
}
