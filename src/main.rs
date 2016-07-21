mod raw;
mod controller;
mod frame;

use std::thread::sleep_ms;
use controller::Controller;

fn main() {
    let controller = Controller::new();

    loop {
        if controller.is_connected() {
            let frame = controller.frame();
            println!("fps = {}", frame.current_fps());
        }
        else {
            println!("Not connected!");
        }
        sleep_ms(150);
    }

    println!("CONNECTED!");
}
