use std::process::Command;

use gtk::{gdk::{prelude::{DisplayExt, MonitorExt}, Display, Monitor}, gio::prelude::ListModelExt, glib::object::Cast, Application};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprMonitor {
    id: u8,
    name: String,
}

#[derive(Debug)]
pub struct FusedMonitorType {
    id: u8,
    name: String,
    gtk_monitor: Monitor
}

pub fn get_monitors() -> Vec<FusedMonitorType> {
    let mut monitors = Vec::<FusedMonitorType>::new();


    let output = Command::new("hyprctl")
        .args(&["monitors", "-j"])
        .output()
        .expect("Failed to run hyprctl");

    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");

    let hypr_monitors: Vec<HyprMonitor> = serde_json::from_str(&stdout).unwrap();
    for monitor in &hypr_monitors {
        println!("hypr_monitor: {:#?}", monitor.name)
    }

    let display = Display::default().expect("no gtk default display");
    let gtk_monitors = display.monitors();

    for i in 0..gtk_monitors.n_items() {
        let obj = gtk_monitors.item(i).expect("Invalid monitor");
        let m = obj.downcast::<Monitor>().expect("failed to cast to monitor type");
        println!("gtk_monitor: {:#?}", m.connector());

        let matching_monitor = hypr_monitors.iter().find(|hypr_m| { 
            m.connector().as_ref().map(|s| s.as_str()) == Some(hypr_m.name.as_str())
        });

        if let Some(val) = matching_monitor {
            monitors.push(FusedMonitorType { id: val.id, name: val.name.clone(), gtk_monitor: m });
        }
    }


    return monitors;
}
