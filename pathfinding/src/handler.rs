/// Handle events
use nannou::prelude::*;
use nannou::winit::event;

use super::{Model, State};

pub fn handle_click(
    _app: &App,
    model: &mut Model,
    button: u32,
    state: event::ElementState,
) {
    if model.state != State::Building {
        return;
    }

    if let event::ElementState::Pressed = state {
        match button {
            1 => (),                                               // left
            2 => model.state = State::Initializing, // scroll wheel press
            3 => model.selected_cell = model.selected_cell.next(), // right
            a => println!("Pressed {}", a),
        }
    }
}

pub fn handle_wheel(model: &mut Model, delta: event::MouseScrollDelta) {
    match delta {
        event::MouseScrollDelta::LineDelta(_x, y) => {
            if y < 0. {
                model.selected_cell = model.selected_cell.next();
            } else {
                model.selected_cell = model.selected_cell.previous();
            }
        }

        event::MouseScrollDelta::PixelDelta(_delta) => {}
    }
}
