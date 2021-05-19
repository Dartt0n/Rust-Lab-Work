extern crate gtk;
use gtk::prelude::*;
use crate::globals::SORTS;

pub fn init_gui(app: &gtk::Application) {
    let main_window = gtk::ApplicationWindowBuilder::new()
        .width_request(1200)
        .height_request(650)
        .build();
    let field = gtk::FixedBuilder::new()
        .width_request(1200)
        .height_request(650)
        .build();
    main_window.add(&field);
    let sort_chooser = gtk::ComboBoxBuilder::new()
        .width_request(250)
        .height_request(50)
        .build();

    let sort_names = {
        let mut x = Vec::<&str>::new();
        for sort in &SORTS {
            x.push(sort.name);
        }
        x
    };
    let mut variants = Vec::<gtk::Label>::new();
    for name in sort_names {
        variants.push(gtk::Label::new(Some(name)));
    }

    for variant in variants {
        sort_chooser.add(&variant);
    }

    field.put(&sort_chooser, 10, 10);

    let recalculate_button = gtk::ButtonBuilder::new()
        .width_request(250)
        .height_request(50)
        .build();

    field.put(&recalculate_button, 10, 70);

    main_window.set_application(Some(app));
    main_window.show_all();
}