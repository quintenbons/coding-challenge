/// Handle events
use nannou::winit::event;

use super::Model;

pub fn handle_click(model: &mut Model, button: u32, state: event::ElementState) {
    if let event::ElementState::Pressed = state {
        match button {
            1 => if model.ray_bounce < u8::MAX { model.ray_bounce += 1 },
            3 => if model.ray_bounce > 1 { model.ray_bounce -= 1 },
            _ => (),
        }
    }
}

pub fn handle_wheel(model: &mut Model, delta: event::MouseScrollDelta) {
    match delta {
        event::MouseScrollDelta::LineDelta(_, y) => {
            if y > 0. && model.ray_count < u32::MAX {
                model.ray_count += 1;
            } else if model.ray_count > 1 {
                model.ray_count -= 1;
            }
        },

        event::MouseScrollDelta::PixelDelta(delta) => {
            if delta.y > 0. && model.ray_count < u32::MAX {
                model.ray_count += 1;
            } else if model.ray_count > 1 {
                model.ray_count -= 1;
            }
        },
    }
}
