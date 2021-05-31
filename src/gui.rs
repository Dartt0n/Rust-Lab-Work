extern crate gtk;

use crate::globals::{run_sorts, SORTS};
use gtk::prelude::*;
use std::fs::OpenOptions;
use std::io::Read;

pub fn init_gui(app: &gtk::Application) {
    let main_window = gtk::ApplicationWindowBuilder::new()
        .width_request(920)
        .height_request(850)
        .build();

    let field = gtk::FixedBuilder::new()
        .width_request(920)
        .height_request(850)
        .build();
    main_window.add(&field);

    let sort_chooser = gtk::ComboBoxTextBuilder::new()
        .width_request(445)
        .height_request(50)
        .build();
    sort_chooser.append(Some("Default"), "Выберите сортировку");
    for (index, sort) in SORTS.iter().enumerate() {
        sort_chooser.append(Some(&format!("{}", index)), sort.name)
    }
    sort_chooser.set_active_id(Some("Default"));

    field.put(&sort_chooser, 10, 10);

    let recalculate_button = gtk::ButtonBuilder::new()
        .width_request(445)
        .height_request(50)
        .build();
    recalculate_button.add(&gtk::Label::new(Some("Пересчитать все")));

    recalculate_button.connect_button_press_event(|_btn, _event| {
        run_sorts();
        gtk::Inhibit(false)
    });

    field.put(&recalculate_button, 465, 10);

    const DRAWING_WIDTH: f64 = 900.0;
    const DRAWING_HEIGHT: f64 = 760.0;

    let drawing_area = gtk::DrawingAreaBuilder::new()
        .width_request(DRAWING_WIDTH as i32)
        .height_request(DRAWING_HEIGHT as i32)
        .build();
        
    let mutant = drawing_area.clone();
    sort_chooser.connect_changed(move |_cmbx| {mutant.queue_draw()});
    
    drawing_area.connect_draw(move |_x, c| {
        c.set_source_rgb(0.12, 0.12, 0.12);
        c.rectangle(0.0, 0.0, DRAWING_WIDTH, DRAWING_HEIGHT);
        c.fill();
        c.set_source_rgb(1.0, 1.0, 1.0);
        c.set_line_width(0.5);
        c.move_to(10.0, 10.0);
        c.line_to(10.0, DRAWING_HEIGHT - 10.0);
        c.move_to(10.0, DRAWING_HEIGHT - 10.0);
        c.line_to(DRAWING_WIDTH - 10.0, DRAWING_HEIGHT - 10.0);
        c.stroke();
        c.set_font_size(10.0);
        let x_step = (DRAWING_WIDTH - 40.0) / 6.0;
        let sizes = [100.0, 500.0, 1000.0, 5000.0, 10000.0, 50000.0];
        for i in 0..6 {
            c.move_to(x_step * (i as f64 + 1.0) - 10.0, DRAWING_HEIGHT - 13.0);
            c.show_text(&format!("{}", sizes[i]));
            c.stroke();
        }
        c.set_line_width(2.0);
        let mut file = OpenOptions::new().read(true).open("result.csv").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        drop(file);
        for line in data.split("\n") {
            let values = line.split(":").collect::<Vec<&str>>();
            let name = values[0];
            if name != sort_chooser.get_active_text().unwrap() {
                continue;
            }
            let mut times = Vec::new();
            for i in 1..values.len() {
                times.push(values[i].parse::<i32>().unwrap());
            }
            let max_time = times.iter().max().unwrap();
            let min_time = times.iter().min().unwrap();
            let delta = max_time - min_time;
            let y_step = (DRAWING_HEIGHT - 20.0) / (delta as f64);
            c.set_source_rgb(1.0, 1.0, 1.0);
            let mut last_point = (10.0, DRAWING_HEIGHT-10.0);
            for (i, time) in times.iter().enumerate() {
                let x = x_step * (i as f64 + 1.0) - 10.0;
                let y = DRAWING_HEIGHT - 10.0 - y_step * f64::from(*time);
                c.move_to(last_point.0, last_point.1);
                c.line_to(x, y);
                c.move_to(20.0, y);
                c.set_source_rgb(1.0, 1.0, 1.0);
                c.show_text(&format!("{}", time));
                c.set_source_rgb(1.0, 0.3, 0.0);
                last_point = (x, y);
            }
            c.stroke();
            break;
        }

        gtk::Inhibit(false)
    });

    field.put(&drawing_area, 10, 70);

    main_window.set_application(Some(app));
    main_window.show_all();
}
