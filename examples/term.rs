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
            let pointer = pointable.stabilized_tip_position();

            let x = rb.width() / 2;
            let y = rb.height() / 2;

            let x = x as f32 + pointer.x().round();
            let y = y as f32 + (pointer.y().round() - 150.) / -2.;


            let x = x as usize;
            let y = y as usize;

            rb.print(0, 0, RB_NORMAL, Color::Default, Color::Default,
                &format!("px = {}", pointer.x().round())
            );
            rb.print(0, 1, RB_NORMAL, Color::Default, Color::Default,
                &format!("py = {}", (pointer.y().round() - 150.) / -2.)
            );
            rb.print(0, 2, RB_NORMAL, Color::Default, Color::Default,
                &format!("x = {}", x)
            );
            rb.print(0, 3, RB_NORMAL, Color::Default, Color::Default,
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
