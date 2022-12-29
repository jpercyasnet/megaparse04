extern crate gtk;
extern crate exif;
extern crate chrono;
extern crate regex;
extern crate walkdir;
extern crate gio;
use gtk::prelude::*;
// use std::env;
use build_ui::build_ui;

mod build_ui;

fn main() {
//    env::set_var("RUST_BACKTRACE", "1");
    let application =
        gtk::Application::new(Some("org.megaparse04"), Default::default());

    application.connect_activate(build_ui);

    application.run();

}
