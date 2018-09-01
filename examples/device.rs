extern crate leap;
extern crate rustbox;

use leap::Controller;
use rustbox::Event::KeyEvent;
use rustbox::RB_NORMAL;
use rustbox::{Color, Key, RustBox};
use std::time::Duration;

fn main() {
    let controller = Controller::new();
    let rb = RustBox::init(Default::default()).expect("rustbox init");

    loop {
        rb.clear();
        show_device_infos(&rb, &controller);

        if let Ok(KeyEvent(Key::Char('q'))) = rb.peek_event(Duration::from_millis(100), false) {
            break;
        }

        rb.present();
    }
}

fn show_device_infos(rb: &RustBox, controller: &Controller) {
    let x = 0;
    let mut y = 0;
    let mut y = || {
        y += 1;
        y - 1
    };
    let mut println = |msg: &str| rb.print(x, y(), RB_NORMAL, Color::Default, Color::Default, msg);

    let devices = controller.devices();

    if devices.is_empty() {
        println("No tracking device available.");
        return;
    }

    for device in devices.iter() {
        println(&format!("serial_number = {:?}", device.serial_number()));
        println(&format!("is_valid = {}", device.is_valid()));
        println(&format!("is_embedded = {}", device.is_embedded()));
        println(&format!("is_streaming = {}", device.is_streaming()));
        println(&format!("range = {}", device.range()));
        println(&format!("baseline = {}", device.baseline()));
        println(&format!(
            "horizontal_view_angle = {}",
            device.horizontal_view_angle()
        ));
        println(&format!(
            "vertical_view_angle = {}",
            device.vertical_view_angle()
        ));
        // println(&format!("is_lighting_bad = {}", device.is_lighting_bad()));
        // println(&format!("is_smudged = {}", device.is_smudged()));
        println("")
    }
}
