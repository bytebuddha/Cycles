macro_rules! or_panic {
    (Res $expr:expr) => {
        match $expr {
            Ok(item) => item,
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    };
    ($expr:expr) => {
        match $expr {
            Some(item) => item,
            None => {
                panic!("Called unwrap on None");
            }
        }
    };
}

macro_rules! add_stylesheet {
    ($file:expr, $widget:expr) => {
        let style_context = $widget.get_style_context();
        let style = include_bytes!($file);
        let provider = gtk::CssProvider::new();
        or_panic!(Res provider.load_from_data(style));
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    };
    (raw $file:expr, $widget:expr) => {
        let style_context = $widget.get_style_context();
        let provider = gtk::CssProvider::new();
        or_panic!(Res provider.load_from_data($file));
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    };
}
