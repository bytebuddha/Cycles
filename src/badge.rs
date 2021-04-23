use gtk::prelude::*;
use relm::{connect, Relm, Widget, Update};
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {}

pub struct Model {
    relm: Relm<Badge>,
    text: String
}

#[widget]
impl Widget for Badge {
    fn init_view(&mut self) {
        add_stylesheet!("../css/badge.css", self.widgets.label);
    }

    fn model(relm: &Relm<Self>, text: String) -> Model {
        Model {
            relm: relm.clone(),
            text
        }
    }

    fn update(&mut self, _: Msg) {}

    view! {
        #[name="label"]
        gtk::Label {
            label: &self.model.text
        }
    }
}
