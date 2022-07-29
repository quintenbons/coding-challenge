#![allow(dead_code)]
/// Visual pathfinding
mod board;
mod handler;

use board::{CellType, Terrain};
use handler::*;
use nannou::prelude::*;
use nannou::winit::event;
use std::collections::HashSet;
use std::time::Duration;

const PAUSE_DURATION: Duration = Duration::from_millis(50);

#[derive(PartialEq)]
enum State {
    Building,
    Initializing,
    Running,
}

pub struct Model {
    terrain: Terrain,
    next_tick: Duration,
    state: State,
    selected_cell: CellType,
    seen: HashSet<(usize, usize)>,
    current: Vec<(usize, usize)>,
    next: Vec<(usize, usize)>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(0.2));
    let window_rect = app.window_rect();

    Model {
        terrain: Terrain::new(window_rect.w(), window_rect.h()),
        next_tick: Duration::from_millis(500),
        state: State::Building,
        selected_cell: CellType::Wall,
        seen: HashSet::new(),
        current: Vec::new(),
        next: Vec::new(),
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::DeviceEvent(_id, event) = event {
        match event {
            event::DeviceEvent::Button { button, state } => {
                handle_click(app, model, button, state);
            }
            event::DeviceEvent::MouseWheel { delta } => {
                handle_wheel(model, delta);
            }
            _ => (),
        }
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    // Fluid cell placement
    if State::Building == model.state
        && app
            .mouse
            .buttons
            .pressed()
            .any(|(button, _)| button == MouseButton::Left)
    {
        model
            .terrain
            .place(app.mouse.position(), model.selected_cell);
    }

    if model.next_tick > update.since_start {
        return;
    }
    model.next_tick += PAUSE_DURATION;

    match model.state {
        State::Initializing => {
            println!("Starting pathfinding");

            let start_pos = model.terrain.start_stop[0].unwrap();
            model.seen.insert(start_pos);
            model.current.push(start_pos);

            model.state = State::Running;
        }

        State::Running => {
            // get next cell if any
            if model.current.is_empty() {
                if model.next.is_empty() {
                    println!("Search is over, there is no path");
                    model.seen.drain();
                    model.state = State::Building;
                } else {
                    println!("Current is now empty");
                    model.current = model.next.drain(..).collect();
                }
                return;
            }

            // get all neighbours
            let (i, j) = model.current.pop().unwrap();
            let neighbours = model.terrain.get_neighbours(i, j);

            for (k, l) in neighbours {
                if model.seen.contains(&(k, l)) {
                    continue;
                }

                let mut neighbour_cell = model.terrain.get_mut(k, l).unwrap();
                match neighbour_cell.ctype {
                    CellType::Empty => {
                        neighbour_cell.ctype = CellType::Start;
                        model.seen.insert((k, l));
                        model.next.push((k, l));
                    }
                    CellType::Stop => {
                        println!("Search is over, found exit");
                        model.seen.drain();
                        model.state = State::Building;
                        break;
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.terrain.draw(&draw);

    // Write to the window frame
    draw.to_frame(app, &frame).expect("Could not draw to frame");
}
