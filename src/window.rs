use gtk::{gdk::Display, prelude::{BoxExt, GridExt, GtkWindowExt, WidgetExt}, style_context_add_provider_for_display, Align, Application, ApplicationWindow, Box, CssProvider, Grid, Label, Orientation, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk_layer_shell::{Edge, LayerShell, Layer};

use super::modules::session;

pub fn start_window(application: &Application) {
    let window = ApplicationWindow::new(application);
    
    /*
        setup bar
     */
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

    /*
        setup base grid / bar
     */
    let bar_grid = Grid::new();
    bar_grid.add_css_class("bar");
    bar_grid.set_halign(Align::Fill);
    

    let left_box = Box::new(Orientation::Horizontal, 0);
    let middle_box = Box::new(Orientation::Horizontal, 0);
    let right_box = Box::new(Orientation::Horizontal, 0);

    left_box.set_halign(Align::Start);
    left_box.set_hexpand(true);
    left_box.add_css_class("island");
    left_box.add_css_class("left");

    middle_box.set_halign(Align::Center);
    middle_box.add_css_class("island");
    middle_box.add_css_class("middle");

    right_box.set_halign(Align::End);
    right_box.set_hexpand(true);
    right_box.add_css_class("island");
    right_box.add_css_class("right");


    let time_label = Label::new(Some("time"));
    let test_label = Label::new(Some("label contents"));

    middle_box.append(&test_label);
    right_box.append(&time_label);

    bar_grid.attach(&left_box, 0, 0, 1, 1);
    bar_grid.attach(&middle_box, 1, 0, 1, 1);
    bar_grid.attach(&right_box, 2, 0, 1, 1);

    /*
        load contents / modules
     */
    let session_item = session::get_module();

    right_box.append(&session_item);

    window.set_child(Some(&bar_grid));

    window.show();
}

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}