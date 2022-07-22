/// Prime spiral playing with polar coordinates
/// Note that selecting only the "prime" numbers
/// Is somewhat irrelevant, it just makes the
/// drawing less dense
mod handler;

use handler::*;
use nannou::prelude::*;
use nannou::winit::event;
use primes::PrimeSet;

const DEFAULT_SCALE: u32 = 10;
const BACKGROUND_COLOR: Rgb<u8> = BLACK;
const ACTIVE_TEXT: bool = false;
const BORDER_WIDTH: f32 = 0.;

pub struct Model {
    scale: u32,
    active_text: bool,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    Model {
        scale: DEFAULT_SCALE,
        active_text: ACTIVE_TEXT,
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

fn draw_frame(draw: &Draw, x: f32, y: f32, size: f32, number: u32, next: u32, model: &Model) {
    let x = x * size;
    let y = y * size;
    let smaller = size - BORDER_WIDTH;

    let string = number.to_string();
    let color = if number == next { BLUE } else { BACKGROUND_COLOR };

    draw.rect().color(WHITE).x_y(x, y).w_h(size, size);

    draw.rect()
        .color(color)
        .x_y(x, y)
        .w_h(smaller, smaller);

    if model.active_text {
        draw.text(&string).color(WHITE).x_y(x, y);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    // Background
    draw.background().color(BACKGROUND_COLOR);

    // Draw Rectangles
    let number = model.scale as f32 * 2. + 1.;
    let stop: u32 = number.pow(2) as u32;
    let size = window.w().max(window.h()) / number;

    let mut x = 0.;
    let mut y = 0.;

    let mut prime_gen = primes::Sieve::new();
    let mut iter_gen = prime_gen.iter();
    let mut next = iter_gen.next().unwrap().try_into().unwrap();

    let mut turn = 0;
    let mut count = 0;

    loop {
        for _ in 0..(turn / 2 + 1) {
            draw_frame(&draw, x, y, size, count, next, &model);

            if count == next {
                next = iter_gen.next().unwrap().try_into().unwrap();
            }

            match turn % 4 {
                0 => x += 1.,
                1 => y += 1.,
                2 => x -= 1.,
                3 => y -= 1.,
                _ => panic!("Modulo operator failed"),
            }

            count += 1;
        }

        turn += 1;
        if count == stop {
            break;
        }
    }

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();
}
