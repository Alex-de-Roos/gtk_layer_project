use gtk::{gio::prelude::{ApplicationExt, ApplicationExtManual}, prelude::{GtkWindowExt, WidgetExt}, Application, ApplicationWindow, Label};
use gtk_layer_shell::{Edge, LayerShell, Layer};


const APP_ID: &str = "hell.gtk_app.widget_1";


fn main() {
    let application = Application::new(Some(APP_ID), Default::default());

     application.connect_activate(|application| {
        start_window(application);
    });

    application.run();
}

fn start_window(application: &Application) {
    let window = ApplicationWindow::new(application);
    
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.auto_exclusive_zone_enable();

     let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, false),
        (Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }


    // Set up a widget
    let label = Label::new(Some("label contents"));
    // label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    window.set_child(Some(&label));
    window.show();
}