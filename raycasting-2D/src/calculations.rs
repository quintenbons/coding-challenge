/// Caclulations for raytracing
use super::{Wall, MAX_DIST, RAY_ALPHA, WALL_WEIGHT};
use nannou::draw::Draw;
use nannou::prelude::*;
use nannou::rand::{self, Rng};

fn intersect(line1: &Wall, line2: &Wall) -> Option<Vec2> {
    let x1 = line1.p0.x;
    let y1 = line1.p0.y;
    let x2 = line1.p1.x;
    let y2 = line1.p1.y;

    let x3 = line2.p0.x;
    let y3 = line2.p0.y;
    let x4 = line2.p1.x;
    let y4 = line2.p1.y;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if denom.is_zero() {
        None
    } else {
        let num_x = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let num_y = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        Some(Vec2::new(num_x / denom, num_y / denom))
    }
}

fn mix_colors(col1: Rgba<u8>, col2: Rgba<u8>) -> Rgba<u8> {
    let r = col1.red as u16 / 3 + col2.red as u16 * 2 / 3;
    let r = r.clamp(0, 255) as u8;

    let g = col1.green as u16 / 3 + col2.green as u16 * 2 / 3;
    let g = g.clamp(0, 255) as u8;

    let b = col1.blue as u16 / 3 + col2.blue as u16 * 2 / 3;
    let b = b.clamp(0, 255) as u8;

    rgba8(r, g, b, RAY_ALPHA)
}

/// Directly draws the ray from point, direction and color
pub fn draw_ray(
    draw: &Draw,
    start: &Vec2,
    dir: &Vec2,
    color: Rgba<u8>,
    walls: &Vec<Wall>,
    depth: u8,
) -> Result<(), &'static str> {
    let res = cast_ray(start, dir, walls)?;

    if let Some((end, wall)) = res {
        draw.line()
            .start(*start)
            .end(end)
            .rgba8(color.red, color.green, color.blue, RAY_ALPHA);

        if depth > 1 {
            let start = end;
            let end = wall.bounce(dir);
            let color = mix_colors(color, wall.color);

            draw_ray(draw, &start, &end, color, walls, depth - 1)
        } else {
            Ok(())
        }
    } else {
        draw.line()
            .start(*start)
            .end(*start + MAX_DIST * *dir)
            .rgba8(color.red, color.green, color.blue, RAY_ALPHA);
        Ok(())
    }
}

/// Gives the stop point of a ray given a start,
/// an angle, and the walls
pub fn cast_ray<'a>(
    start: &Vec2,
    dir: &Vec2,
    walls: &'a Vec<Wall>,
) -> Result<Option<(Vec2, &'a Wall)>, &'static str> {
    let ray = Wall::from_points(*start, *start + *dir);

    let mut dist: f32 = 0.;
    let mut closest = None;
    let mut too_close_count = 0;

    for wall in walls {
        match intersect(&ray, wall) {
            None => continue,
            Some(intersection) => {
                let new_dir = intersection - *start;

                let right_dir = new_dir.angle_between(*dir).abs() < 0.001;
                let new_dist = new_dir.length();

                if new_dist < WALL_WEIGHT {
                    too_close_count += 1;
                    if too_close_count == 2 {
                        return Err("Possibly hit a corner, aborting bounce");
                    }
                    continue;
                }

                if right_dir && new_dist < MAX_DIST && !wall.rejects(intersection) {
                    if closest.is_none() || new_dist < dist {
                        dist = new_dist;
                        closest = Some((intersection, wall));
                    }
                }
            }
        }
    }

    Ok(closest)
}

/// Gives a point (Vec2) at random in the window
fn generate_point(rect: Rect) -> Vec2 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(rect.left()..rect.right());
    let y = rng.gen_range(rect.bottom()..rect.top());

    Vec2::new(x, y)
}

fn generate_color() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    rgba(rng.gen(), rng.gen(), rng.gen(), 255)
}

/// Gives a vector of number walls
pub fn generate_walls(number: usize, window_rect: Rect) -> Vec<Wall> {
    (0..number)
        .map(|_| {
            let p0 = generate_point(window_rect);
            let p1 = generate_point(window_rect);
            let color = generate_color();

            Wall::new(p0, p1, color)
        })
        .collect()
}
