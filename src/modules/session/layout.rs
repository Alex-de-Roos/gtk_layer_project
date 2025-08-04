use gtk::{gio::{prelude::ActionMapExt, SimpleAction, SimpleActionGroup}, glib::object::IsA, prelude::{ActionableExt, BoxExt, PopoverExt, WidgetExt}, Box, Button, MenuButton, Popover, Widget};

use super::functions;

pub fn get_module() -> impl IsA<Widget> {
    let container = crate::utils::builders::build_module_container("session");

    let menu_button = MenuButton::new();
    menu_button.set_icon_name("system-shutdown");

    let action_group = SimpleActionGroup::new();
    
    let popover = Popover::new();
    popover.set_has_arrow(false);
    let popover_box = Box::new(gtk::Orientation::Vertical, 0);

    /*
        lock
     */
    let lock_button = Button::from_icon_name("system-lock-screen");
    lock_button.set_action_name(Some("module_session_actions.lock"));
    let lock_action = SimpleAction::new("lock", None);
    lock_action.connect_activate(|_, _| {
        functions::lock();
    });
    action_group.add_action(&lock_action);

    /*
        reboot
     */
    let reboot_button = Button::from_icon_name("system-reboot");
    reboot_button.set_action_name(Some("module_session_actions.reboot"));
    let reboot_action = SimpleAction::new("reboot", None);
    reboot_action.connect_activate(|_, _| {
        functions::reboot();
    });
    action_group.add_action(&reboot_action);

    /*
        power_off
     */
    let power_off_button = Button::from_icon_name("system-shutdown");
    power_off_button.set_action_name(Some("module_session_actions.power_off"));
    let power_off_action = SimpleAction::new("power_off", None);
    power_off_action.connect_activate(|_, _| {
        functions::power_off();
    });
    action_group.add_action(&power_off_action);


    popover_box.append(&lock_button);
    popover_box.append(&reboot_button);
    popover_box.append(&power_off_button);

    popover.set_child(Some(&popover_box));

    menu_button.insert_action_group("module_session_actions", Some(&action_group));
    menu_button.set_popover(Some(&popover));
    container.append(&menu_button);

    return container;
}