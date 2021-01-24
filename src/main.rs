mod game;
mod agent;

extern crate gio;
extern crate gtk;

use std::env::args;
use gio::prelude::*;

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        game::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
