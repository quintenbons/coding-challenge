/// Simple 2D raytracing and display
mod calculations;
mod handler;
mod wall;

use handler::*;
use calculations::*;
use nannou::prelude::*;
use nannou::winit::event;
use wall::Wall;

const WALL_COUNT: usize = 10;
const WALL_WEIGHT: f32 = 5.;

const MAX_DIST: f32 = 10000.;

const RAY_COUNT: u32 = 1000;
const RAY_ALPHA: u8 = 5;
const RAY_BOUNCE: u8 = 3;

pub struct Model {
    walls: Vec<Wall>,
    ray_count: u32,
    ray_bounce: u8,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    let window_rect = app.window_rect();
    let walls = generate_walls(WALL_COUNT, window_rect);

    Model {
        walls,
        ray_count: RAY_COUNT,
        ray_bounce: RAY_BOUNCE,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::DeviceEvent(id, event) = event {
        match event {
            event::DeviceEvent::Button { button, state } => {
                handle_click(model, button, state);
            },
            event::DeviceEvent::MouseWheel { delta } => {
                handle_wheel(model, delta);
            }
            _a => {
                //
            },
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Background
    draw.background().color(BLACK);

    // Walls
    for wall in &model.walls {
        draw.line()
            .start(wall.p0)
            .end(wall.p1)
            .color(wall.color)
            .weight(WALL_WEIGHT);
    }

    // Point
    let mouse_pos = app.mouse.position();
    draw.ellipse().color(YELLOW).xy(mouse_pos).radius(3.);

    // Rays
    let default_color: Rgba<u8> = rgba8(255, 255, 255, RAY_ALPHA);

    for i in 0..model.ray_count {
        let angle: f32 = (i as f32 / model.ray_count as f32) * 2. * PI;
        let dir = Vec2::new(angle.cos(), angle.sin());

        let _ = draw_ray(
            &draw,
            &mouse_pos,
            &dir,
            default_color,
            &model.walls,
            model.ray_bounce,
        );
    }

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();
}
