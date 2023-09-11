/// Prime spiral showing non-random patterns
mod penguin_math;

use itertools::Itertools;
use penguin_math::Model;
use nannou::{prelude::*, winit::event::DeviceEvent, event::ElementState};

const DEFAULT_PENGUIN_NB: u32 = 10;
const DEFAULT_SPEED: f32 = 5.;
const DEFAULT_RADIUS: f32 = 200.;

const PENGUIN_SIZE: f32 = 15.;
const TRACE_WEIGHT: f32 = 2.;
const BACKGROUND_COLOR: Rgb<u8> = BLACK;

fn main() {
    nannou::app(model).simple_window(view).update(update).event(event).run();
}

fn model(_app: &App) -> Model {
    Model::new(DEFAULT_PENGUIN_NB, DEFAULT_SPEED, DEFAULT_RADIUS)
}

fn debug_print(model: &Model, button: u32, state: &ElementState) {
    println!("button: {}", button);
    println!("state: {:?}", state);
    println!("Model debug : {:?}", model);
    println!();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.tick();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::DeviceEvent(_id, d_event) = event {
        match d_event {
            DeviceEvent::Button { button, state } => {
                debug_print(model, button, &state);
            }
            _a => {
                //
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let _window = app.window_rect();

    // Background
    draw.background().color(BACKGROUND_COLOR);

    for (penguin, trace) in model.positions().iter().zip(model.trace().iter()) {
        // penguin
        draw.ellipse()
            .color(RED)
            .w_h(PENGUIN_SIZE, PENGUIN_SIZE)
            .xy(penguin.clone());

        // trace
        // for (cur, next) in Itertools::tuple_windows(trace.iter()) {
        //     draw.line()
        //         .color(RED)
        //         .weight(TRACE_WEIGHT)
        //         .start(cur.clone())
        //         .end(next.clone());
        // }
    }

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();
}
