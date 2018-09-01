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
        let frame = controller.frame();
        if let Some(hand) = frame.hands().frontmost() {
            let dir = hand.direction();

            rb.print(
                0,
                0,
                RB_NORMAL,
                Color::Default,
                Color::Default,
                &format!("id = {}", hand.id()),
            );

            rb.print(
                0,
                1,
                RB_NORMAL,
                Color::Default,
                Color::Default,
                &format!(" yaw = {:.2}", dir.yaw()),
            );

            rb.print(
                0,
                2,
                RB_NORMAL,
                Color::Default,
                Color::Default,
                &format!("pitch = {:.2}", dir.pitch()),
            );

            rb.print(
                0,
                3,
                RB_NORMAL,
                Color::Default,
                Color::Default,
                &format!(" roll = {:.2}", dir.roll()),
            );
        };

        if let Ok(KeyEvent(Key::Char('q'))) = rb.peek_event(Duration::from_millis(150), false) {
            break;
        }

        rb.present();
        rb.clear();
    }
}
