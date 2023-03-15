use gtk::{gdk, gio};

pub fn initialize_icons() {
    gio::resources_register_include!("resources.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/relm4/icons/");
}

#[cfg(all(test, feature = "relm4"))]
mod test {
    use super::*;

    #[test]
    fn test() {
        gtk::init().unwrap();

        initialize_icons();

        let display = gdk::Display::default().unwrap();
        let theme = gtk::IconTheme::for_display(&display);
        assert!(theme.has_icon("relm4-symbolic"));
    }
}
