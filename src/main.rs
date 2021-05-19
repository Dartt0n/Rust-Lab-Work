mod sorts;
mod typed_files;
mod gui;
mod globals;

use crate::typed_files::IntegerFile;
use std::fs::OpenOptions;
use std::io::Result;

extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use crate::gui::init_gui;
use crate::globals::SORTS;


fn run_sorts() {
    let sizes = [100, 500, 1_000, 5_000, 10_000, 50_000];

    for sort in &SORTS {
        let mut files = Vec::<String>::new();
        for size in &sizes {
            let name = format!("data/d{}.dat", size);
            let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(&name).unwrap();
            file.fill_shuffled(*size).unwrap();
            files.push(name);
        }
        for file in files {
            match (sort.run)(&file) {
                Ok(time) => println!("Сортировка: \"{}\"\n\tФайл: \"{}\"\n\tВремя: {} мкс\n", sort.name, file, time.as_micros()),
                Err(e) => println!("Сортировка: \"{}\" Файл: \"{}\" Ошибка: \"{}\"", sort.name, file, e),
            };
        }
    }
}

fn main() {
    // let app = gtk::Application::new(Some("dartt0n.sorts_stats.app"), Default::default())
    //    .expect("Failed to start application");
    //
    // app.connect_activate(init_gui);
    //
    // app.run(&std::env::args().collect::<Vec<_>>());
    run_sorts();
}
