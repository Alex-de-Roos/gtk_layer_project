use gtk::{gio::prelude::{ApplicationExt, ApplicationExtManual}, Application};

mod window;
use window::{start_window, load_css};
mod modules;
mod utils;

const APP_ID: &str = "hell.gtk_item._1";

fn main() {
    let application = Application::new(Some(APP_ID), Default::default());


    application.connect_startup(|_| load_css());
    application.connect_activate(|application| {
        start_window(application);
    });

    application.run();
}

