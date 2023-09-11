/// Prime spiral showing non-random patterns
mod penguin_math;

use itertools::Itertools;
use penguin_math::Model;
use nannou::{prelude::*, winit::event::DeviceEvent, event::ElementState};

const DEFAULT_PENGUIN_NB: u32 = 10;
const DEFAULT_SPEED: f32 = 100.;
const DEFAULT_RADIUS: f32 = 400.;

const PENGUIN_SIZE: f32 = 15.;
const TRACE_WEIGHT: f32 = 2.;
const BACKGROUND_COLOR: Rgb<u8> = BLACK;

const COLORS: [Rgb<u8>; 6] = [
    RED,
    GREEN,
    BLUE,
    YELLOW,
    MAGENTA,
    CYAN,
];

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

    let pos_iter = model.positions().iter();
    let trace_iter = model.trace().iter();
    let col_iter = COLORS.iter().cycle();

    for ((penguin, trace), col) in pos_iter.zip(trace_iter).zip(col_iter) {
        let real_col = Rgba::new(col.red, col.green, col.blue, 50);

        // trace
        for (cur, next) in Itertools::tuple_windows(trace.iter()) {
            draw.line()
                .color(real_col)
                .weight(TRACE_WEIGHT)
                .start(cur.clone())
                .end(next.clone())
                .z(1.);
        }

        // penguin
        draw.ellipse()
            .color(real_col)
            .w_h(PENGUIN_SIZE, PENGUIN_SIZE)
            .xy(penguin.clone())
            .z(2.);
    }
    for t in model.trace() {
        assert_eq!(t.len(), model.trace().first().unwrap().len())
    }

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();
}
