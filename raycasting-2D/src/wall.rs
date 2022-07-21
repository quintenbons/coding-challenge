// Simple wall structure
use nannou::prelude::*;

pub struct Wall {
    pub p0: Vec2,
    pub p1: Vec2,
    pub color: Rgba<u8>,
}

impl Wall {
    /// Checks if point is close enough from the wall
    /// to be considered "on the wall"
    pub fn rejects(&self, point: Vec2) -> bool {
        let dir1 = self.p0 - self.p1;
        let dir2 = self.p0 - point;

        (dir1.angle_between(dir2).abs() - PI).abs() <= 0.01 || dir1.length() < dir2.length()
    }

    /// New default colored wall
    pub fn from_points(p0: Vec2, p1: Vec2) -> Wall {
        Wall {
            p0,
            p1,
            color: rgba(255, 255, 255, 255),
        }
    }

    /// New wall
    pub fn new(p0: Vec2, p1: Vec2, color: Rgba<u8>) -> Wall {
        Wall { p0, p1, color }
    }

    /// Gives the new transformet vector after an elastic
    /// bounce off the wall. The dir vector needs to be
    /// normalized
    pub fn bounce(&self, dir: &Vec2) -> Vec2 {
        let wall_norm = (self.p0 - self.p1).perp().normalize();
        let compensate = dir.project_onto_normalized(wall_norm);

        *dir - 2. * compensate
    }
}
