use gtk::prelude::*;
use relm::{connect, Relm, Widget, Update};
use relm_derive::Msg;

#[derive(Msg)]
pub enum Msg {
    Quit,
    About,
    Import,
    Export
}

pub struct Widgets {
    container: gtk::Popover,
    about_btn: gtk::Button,
    quit_btn: gtk::Button,
    import_btn: gtk::Button,
    export_btn: gtk::Button
}

pub struct Menu {
    model: Model,
    widgets: Widgets,
}

pub struct Model {
    relm: Relm<Menu>,
    btn: gtk::Button
}

impl Update for Menu {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = gtk::Button;
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(relm: &Relm<Self>, btn: gtk::Button) -> Model {
        Model {
            relm: relm.clone(),
            btn
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => {},
            Msg::About => {},
            Msg::Import => {},
            Msg::Export => {}
        }
    }
}

impl Widget for Menu {
    // Specify the type of the root widget.
    type Root = gtk::Popover;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.container.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("../ui/menu.ui");
        let builder = gtk::Builder::from_string(glade_src);
        let container: gtk::Popover = or_panic!(builder.get_object("menuPopover"));
        let quit_btn: gtk::Button = or_panic!(builder.get_object("quitButton"));
        let about_btn: gtk::Button = or_panic!(builder.get_object("aboutButton"));
        let export_btn: gtk::Button = or_panic!(builder.get_object("exportButton"));
        let import_btn: gtk::Button = or_panic!(builder.get_object("importButton"));

        connect!(_relm, quit_btn, connect_clicked(_), Msg::Quit);
        connect!(_relm, about_btn, connect_clicked(_), Msg::About);
        connect!(_relm, import_btn, connect_clicked(_), Msg::Import);
        connect!(_relm, export_btn, connect_clicked(_), Msg::Export);
        container.set_relative_to(Some(&model.btn));
        container.show_all();

        Menu {
            model,
            widgets: Widgets {
                container,
                quit_btn,
                about_btn,
                import_btn,
                export_btn
            },
        }
    }
}
