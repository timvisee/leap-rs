extern crate leap;

use std::thread::sleep;
use std::time::Duration;

use leap::Controller;

fn main() {
    let controller = Controller::new();

    loop {
        if controller.is_connected() {
            let frame = controller.frame();
            println!(
                "fps = {}, pointables = {}",
                frame.current_fps(),
                frame.pointables().len()
            );
            for pointable in frame.pointables().iter() {
                let stp = pointable.stabilized_tip_position();
                println!(
                    "[p]: id = {id}, td = {td:.1}, stp = ({x:.1}, {y:.1}, {z:.1})",
                    id = pointable.id(),
                    td = pointable.touch_distance(),
                    x = stp.x(),
                    y = stp.y(),
                    z = stp.z(),
                );
            }
        } else {
            println!("Not connected!");
        }
        sleep(Duration::from_millis(150));
    }
}
