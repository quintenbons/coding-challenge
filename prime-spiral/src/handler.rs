/// Handle events
use nannou::winit::event;

use super::Model;

pub fn handle_click(model: &mut Model, button: u32, state: event::ElementState) {
    if let event::ElementState::Pressed = state {
        match button {
            1 => model.active_text = !model.active_text,
            _ => (),
        }
    }
}

pub fn handle_wheel(model: &mut Model, delta: event::MouseScrollDelta) {
    match delta {
        event::MouseScrollDelta::LineDelta(_, y) => {
            let new_scale = if y > 0. {
                model.scale + 1
            } else {
                model.scale - 1
            };

            model.scale = new_scale.clamp(3, u32::MAX / 2);
        }

        event::MouseScrollDelta::PixelDelta(delta) => {
            let new_scale = if delta.y > 0. {
                model.scale + 1
            } else {
                model.scale - 1
            };

            model.scale = new_scale.clamp(3, u32::MAX / 2);
        }
    }
}
