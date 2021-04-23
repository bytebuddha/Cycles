use gtk::prelude::*;
use relm::{connect, Relm, Widget, Update};
use relm_derive::Msg;

#[derive(Msg)]
pub enum Msg {
    Quit
}

pub struct Widgets {
    container: gtk::AboutDialog
}

pub struct About {
    model: Model,
    widgets: Widgets,
}

pub struct Model {
    relm: Relm<About>,
    win: libhandy::ApplicationWindow
}

impl Update for About {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = libhandy::ApplicationWindow;
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(relm: &Relm<Self>, win: libhandy::ApplicationWindow) -> Model {
        Model {
            relm: relm.clone(),
            win
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => {}
        }
    }
}

impl Widget for About {
    // Specify the type of the root widget.
    type Root = gtk::AboutDialog;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.container.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("../ui/about.ui");
        let builder = gtk::Builder::from_string(glade_src);
        let container: gtk::AboutDialog = or_panic!(builder.get_object("aboutDialog"));

        container.set_transient_for(Some(&model.win));
        container.set_attached_to(Some(&model.win));

        container.show_all();

        About {
            model,
            widgets: Widgets {
                container
            },
        }
    }
}
