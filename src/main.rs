use gtk::{gio::prelude::{ApplicationExt, ApplicationExtManual}, Application};

mod window;
use window::{start_window, load_css};
mod modules;
mod utils;
mod monitors;

const APP_ID: &str = "hell.gtk_item._1";

fn main() {
    let application = Application::new(Some(APP_ID), Default::default());


    application.connect_startup(|_| load_css());

    application.connect_activate(|application| {

        let monitors = monitors::get_monitors();
        for monitor in monitors {
            println!("monitor: {:#?}", monitor);
            start_window(application, &monitor);
        }

    });

    application.run();
}

