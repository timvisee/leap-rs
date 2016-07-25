extern crate leap;
extern crate rustbox;

use std::time::Duration;
use leap::Controller;
use rustbox::{RustBox,Key, Color};
use rustbox::Event::KeyEvent;
use rustbox::RB_NORMAL;

// TODO: Properly map leap motion space to terminal space

fn main() {
    let controller = Controller::new();
    let rb = RustBox::init(Default::default()).expect("rustbox init");

    loop {
        let frame = controller.frame();
        if let Some(pointable) = frame.pointables().frontmost() {
            let ibox = frame.interaction_box();
            let point = pointable.stabilized_tip_position();
            let point = ibox.normalize_point_clamp(&point, false);

            let mut x = rb.width() as f32;
            let mut y = rb.height() as f32;

            x *=       point.x();
            y *= 1.0 - point.y();

            let x = x.round() as usize;
            let y = y.round() as usize;

            rb.print(0, 0, RB_NORMAL, Color::Default, Color::Default,
                &format!("x = {}", x)
            );
            rb.print(0, 1, RB_NORMAL, Color::Default, Color::Default,
                &format!("y = {}", y)
            );

            rb.print_char(x, y, RB_NORMAL, Color::Default, Color::Default, '#');
        }

        if let Ok(KeyEvent(Key::Char('q'))) = rb.peek_event(Duration::from_millis(1), false) {
            break
        }

        rb.present();
        rb.clear();
    }
}
