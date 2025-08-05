use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Monitor {
    id: u8,
    name: String,
}

pub fn get_monitors() -> () {
    let output = Command::new("hyprctl")
        .args(&["monitors", "-j"])
        .output()
        .expect("Failed to run hyprctl");

    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");

    let monitors: Vec<Monitor> = serde_json::from_str(&stdout).unwrap();

    println!("{:#?}", stdout);

    for monitor in monitors {
        println!("monitor: {:#?}", monitor.name)
    }
}
