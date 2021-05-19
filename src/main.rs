mod globals;
mod gui;
mod sorts;
mod typed_files;

use crate::typed_files::IntegerFile;
use std::fs::OpenOptions;
use std::io::Result;

extern crate gtk;
use crate::globals::SORTS;
use crate::gui::init_gui;
use gio::prelude::*;
use gtk::prelude::*;

fn run_sorts() {
    let sizes = [100, 500, 1_000, 5_000, 10_000, 50_000];

    for sort in &SORTS {
        let mut files = Vec::<String>::new();
        for size in &sizes {
            let name = format!("data/d{}.dat", size);
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&name)
                .unwrap();
            file.fill_shuffled(*size).unwrap();
            files.push(name);
        }
        for size in &sizes {
            match (sort.run)(&format!("data/d{}.dat", size)) {
                Ok(time) => {
                    println!("{} | {} | {}", sort.name, size, time.as_micros())
                }
                Err(e) => println!("{}", e),
            };
        }
    }
}

fn main() {
    let app = gtk::Application::new(None, Default::default()).expect("Failed to start application");

    app.connect_activate(init_gui);

    app.run(&std::env::args().collect::<Vec<_>>());
}
