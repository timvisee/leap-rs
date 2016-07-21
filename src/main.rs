mod raw;
mod controller;
mod frame;
mod pointable;

use std::thread::sleep_ms;
use controller::Controller;

fn main() {
    let controller = Controller::new();

    loop {
        if controller.is_connected() {
            let frame = controller.frame();
            println!("fps = {}, pointables = {}", frame.current_fps(), frame.pointables().count());
            for pointable in frame.pointables().iter() {
                print!("[p] ");
            }
            println!("");
        }
        else {
            println!("Not connected!");
        }
        sleep_ms(150);
    }

}
