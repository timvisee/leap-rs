extern crate leap;

use leap::Controller;
use std::thread::sleep_ms;

fn main() {
    let controller = Controller::with_listener(DemoListener);
    loop {
        // Do something else
        sleep_ms(150);
    }
}

struct DemoListener;

impl leap::Listener for DemoListener {
    fn on_connect(&mut self, _: &Controller) {
        println!("Connected!");
    }

    fn on_frame(&mut self, controller: &Controller) {
        // Get the frame, report information
        let frame = controller.frame();
        println!(
            "fps = {}, pointables = {}, fingers = {}",
            frame.current_fps(),
            frame.pointables().len(),
            frame.fingers().len(),
        );

        // Report all pointables (includes fingers)
        for pointable in frame.pointables().iter() {
            let stp = pointable.stabilized_tip_position();
            println!(
                "[p]: id = {}, td = {}, stp = ({}, {}, {})",
                pointable.id(),
                pointable.touch_distance(),
                stp.x(),
                stp.y(),
                stp.z(),
            );
        }

        // Report all fingers
        for finger in frame.fingers().iter() {
            let stp = finger.stabilized_tip_position();
            println!(
                "[f]: id = {}, type = {}, extended = {}, td = {}, stp = ({}, {}, {})",
                finger.id(),
                finger.type_enum(),
                finger.is_extended(),
                finger.touch_distance(),
                stp.x(),
                stp.y(),
                stp.z(),
            );
        }

        println!("--------------------------------");
        sleep_ms(150);
    }
}
