extern crate leap;
extern crate rustbox;

use leap::Controller;
use rustbox::Event::KeyEvent;
use rustbox::RB_NORMAL;
use rustbox::{Color, Key, RustBox};
use std::collections::HashSet;
use std::time::Duration;

fn main() {
    let controller = Controller::new();
    let rb = RustBox::init(Default::default()).expect("rustbox init");
    let mut drawing = HashSet::<(usize, usize)>::new();

    loop {
        let frame = controller.frame();

        for &(x, y) in &drawing {
            rb.print_char(x, y, RB_NORMAL, Color::Default, Color::Default, '*');
        }

        if let Some(hand) = frame.hands().frontmost() {
            let ibox = frame.interaction_box();
            let point = hand.stabilized_palm_position();
            let point = ibox.normalize_point_clamp(&point, false);

            let mut x = rb.width() as f32;
            let mut y = rb.height() as f32;

            x *= point.x();
            y *= 1.0 - point.y();

            let x = x.round() as usize;
            let y = y.round() as usize;

            if hand.pinch_distance() >= 0.7 {
                drawing.insert((x, y));
            }

            rb.print(
                0,
                0,
                RB_NORMAL,
                Color::Default,
                Color::Default,
                &format!("x = {}", x),
            );

            rb.print(
                0,
                1,
                RB_NORMAL,
                Color::Default,
                Color::Default,
                &format!("y = {}", y),
            );

            rb.print(
                0,
                2,
                RB_NORMAL,
                Color::Default,
                Color::Default,
                &format!("pinch = {:.2}", hand.pinch_distance()),
            );

            rb.print_char(x, y, RB_NORMAL, Color::Default, Color::Default, '#');
        }

        if let Ok(KeyEvent(Key::Char('q'))) = rb.peek_event(Duration::from_millis(1), false) {
            break;
        }

        rb.present();
        rb.clear();
    }
}
