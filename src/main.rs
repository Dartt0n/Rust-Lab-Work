mod globals;
mod gui;
mod sorts;
mod typed_files;
extern crate gtk;
use crate::gui::init_gui;
use gio::prelude::*;

fn main() {
    let app = gtk::Application::new(Some("dartt0n.sorts_stats.app"), Default::default())
        .expect("Failed to start application");

    app.connect_activate(init_gui);

    app.run(&std::env::args().collect::<Vec<_>>());
}
