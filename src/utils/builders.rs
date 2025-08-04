use gtk::{prelude::WidgetExt, Box};

pub fn build_module_container(module_name: &str) -> Box{
    let container = Box::new(gtk::Orientation::Horizontal, 0);
    container.add_css_class("module");
    container.add_css_class(&format!("module-{}", module_name));
    
    return container
}