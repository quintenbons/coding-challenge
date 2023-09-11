/// So complicated
use std::{time::{Instant, Duration}, fmt};

use nannou::prelude::{PI, Point2, Vec2};

const TICK_DURATION: Duration = Duration::from_millis(50);

fn move_peng(src: &mut Vec2, dst: Vec2, speed: f32) {
    let relative = dst - *src;
    if relative.length() > speed {
        let unit = relative.normalize();
        *src += unit * speed;
    } else {
        *src = dst;
    }
}

pub struct Model {
    speed: f32,
    polygon_corners: Vec<Point2>,
    penguin_positions: Vec<Point2>,
    trace: Vec<Vec<Point2>>,
    last_tick: Instant,
}

impl Model {
    pub fn new(penguin_nb: u32, speed: f32, radius: f32) -> Model {
        let mut corners = Vec::new();

        for i in 0..penguin_nb {
            let angle = 2.0 * PI * (i as f32) / (penguin_nb as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            corners.push(Point2::new(x, y));
        }

        Model {
            speed,
            trace: Vec::new(),
            penguin_positions: corners.clone(),
            polygon_corners: corners,
            last_tick: Instant::now(),
        }
    }

    pub fn step(&mut self) {
        let mut next_last = self.penguin_positions.last().unwrap().clone();

        for (trace, cur) in self.trace.iter_mut().zip(self.penguin_positions.iter()) {
            trace.push(cur.clone());
        }

        move_peng(&mut next_last, self.penguin_positions[0], self.speed);
        for i in 0..(self.penguin_positions.len() - 1) {
            let next = self.penguin_positions[i+1];
            let cur = &mut self.penguin_positions[i];
            move_peng(cur, next, self.speed);
        }

        *self.penguin_positions.last_mut().unwrap() = next_last;
    }

    pub fn tick(&mut self) {
        if self.last_tick.elapsed() >= TICK_DURATION {
            self.step();
            self.last_tick = Instant::now();
        }
    }

    pub fn positions(&self) -> &Vec<Point2> {
        &self.penguin_positions
    }

    pub fn trace(&self) -> &Vec<Vec<Point2>> {
        &self.trace
    }
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Model {{ penguin_nb: {}, speed: {} }}", self.penguin_positions.len(), self.speed)
    }
}
