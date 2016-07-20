mod raw;
mod controller;

use std::thread::sleep_ms;
use controller::Controller;

fn main() {
    let controller = Controller::new();

    while !controller.is_connected() {
        println!("Not connected!");
        sleep_ms(2000);
    }

    println!("CONNECTED!");
}
