extern crate gtk;
use crate::globals::SORTS;
use glib::types::Type;
use gtk::prelude::*;

pub fn init_gui(app: &gtk::Application) {
    let main_window = gtk::ApplicationWindowBuilder::new()
        .width_request(1415)
        .height_request(850)
        .build();

    let field = gtk::FixedBuilder::new().width_request(1200).height_request(650).build();
    main_window.add(&field);

    let sort_chooser = gtk::ComboBoxTextBuilder::new()
        .width_request(375)
        .height_request(50)
        .build();
    sort_chooser.append(Some("Default"), "Выберите сортировку");
    for (index, sort) in SORTS.iter().enumerate() {
        sort_chooser.append(Some(&*index.to_string()), sort.name)
    }
    sort_chooser.set_active_id(Some("Default"));
    field.put(&sort_chooser, 10, 10);

    let recalculate_button = gtk::ButtonBuilder::new().width_request(375).height_request(50).build();
    recalculate_button.add(&gtk::Label::new(Some("Пересчитать")));
    field.put(&recalculate_button, 10, 70);

    let unchanged_file = gtk::TextViewBuilder::new()
        .width_request(375)
        .height_request(345)
        .build();
    field.put(&unchanged_file, 10, 130);

    let changed_file = gtk::TextViewBuilder::new()
        .width_request(375)
        .height_request(345)
        .build();
    field.put(&changed_file, 10, 485);

    let drawing_area = gtk::DrawingAreaBuilder::new()
        .width_request(1000)
        .height_request(830)
        .build();

    drawing_area.connect_draw(move |_, c| {
        // background
        c.set_source_rgb(0.12, 0.12, 0.12);
        c.rectangle(0.0, 0.0, 1000.0, 830.0);
        c.fill();
        gtk::Inhibit(false)
    });

    field.put(&drawing_area, 395, 10);

    main_window.set_application(Some(app));
    main_window.show_all();
}
