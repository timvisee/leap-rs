extern crate leap;
extern crate rustbox;

use leap::Controller;
use rustbox::Event::KeyEvent;
use rustbox::RB_NORMAL;
use rustbox::{Color, Key, RustBox};
use std::time::Duration;

// TODO: Properly map leap motion space to terminal space

fn main() {
    let controller = Controller::new();
    let rb = RustBox::init(Default::default()).expect("rustbox init");

    loop {
        let frame = controller.frame();
        for pointable in frame.pointables().iter() {
            let ibox = frame.interaction_box();
            let point = pointable.stabilized_tip_position();
            let point = ibox.normalize_point_clamp(&point, false);

            let mut x = rb.width() as f32;
            let mut y = rb.height() as f32;

            x *= point.x();
            y *= 1.0 - point.y();

            let x = x.round() as usize;
            let y = y.round() as usize;

            if x < 2000 && y < 2000 {
                let fg_color = color_from_i32(pointable.id());
                rb.print(
                    x,
                    y,
                    RB_NORMAL,
                    fg_color,
                    Color::Default,
                    &format!("# {}", pointable.id()),
                );
            }
        }

        if let Ok(KeyEvent(Key::Char('q'))) = rb.peek_event(Duration::from_millis(10), false) {
            break;
        }

        rb.present();
        rb.clear();
    }
}

fn color_from_i32(n: i32) -> Color {
    use rustbox::Color::*;
    let n = n.abs() % 6;

    match n {
        0 => Red,
        1 => Green,
        2 => Blue,
        3 => Magenta,
        4 => Yellow,
        5 => Cyan,
        _ => unreachable!(),
    }
}
