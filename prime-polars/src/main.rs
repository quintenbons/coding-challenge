/// Prime spiral playing with polar coordinates
/// Note that selecting only the "prime" numbers
/// Is somewhat irrelevant, it just makes the
/// drawing less dense
mod handler;

use handler::*;
use nannou::prelude::*;
use nannou::winit::event;
use primes::PrimeSet;

const DEFAULT_SCALE: f32 = 1.;
const BACKGROUND_COLOR: Rgb<u8> = BLACK;
const POINT_RADIUS: f32 = 2.;
const POINT_COLOR: Rgb<u8> = WHITE;

pub struct Model {
    scale: f32,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    Model {
        scale: DEFAULT_SCALE,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::DeviceEvent(id, event) = event {
        match event {
            event::DeviceEvent::Button { button, state } => {
                handle_click(model, button, state);
            }
            event::DeviceEvent::MouseWheel { delta } => {
                handle_wheel(model, delta);
            }
            _a => {
                //
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    // Background
    draw.background().color(BACKGROUND_COLOR);

    // Draw Points
    let wc = window.w() * model.scale / 2.;
    let hc = window.h() * model.scale / 2.;
    let stop: f32 = wc.pow(2) + hc.pow(2);
    let stop: u64 = stop.sqrt() as u64;

    println!("{} - {}", stop, window.right());

    let mut prime_gen = primes::Sieve::new();

    for prime in prime_gen.iter() {
        if prime > stop {
            break;
        }

        let n = prime as f32;

        let point = n * Vec2::new(n.cos(), n.sin()) / model.scale;

        draw.ellipse()
            .radius(POINT_RADIUS)
            .xy(point)
            .color(POINT_COLOR);
    }

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();
}
