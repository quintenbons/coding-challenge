/// Board management
use nannou::prelude::*;
use once_cell::sync::Lazy;

const CELL_SIDE: f32 = 100.;

static COLOR_BACKGROUND: Lazy<Rgb8> = Lazy::new(|| Rgb8::new(0x80, 0x80, 0x80));
static COLOR_EMPTY: Lazy<Rgb8> = Lazy::new(|| Rgb8::new(0xFF, 0xFF, 0xFF));
static COLOR_WALL: Lazy<Rgb8> = Lazy::new(|| Rgb8::new(0x30, 0x30, 0x30));
static COLOR_START: Lazy<Rgb8> = Lazy::new(|| Rgb8::new(0x22, 0x66, 0x00));
static COLOR_STOP: Lazy<Rgb8> = Lazy::new(|| Rgb8::new(0xFF, 0x4D, 0x4D));

/// A cell could contain all kind of attributes
#[derive(Copy, Clone)]
pub enum CellType {
    Empty,
    Wall,
    Start,
    Stop,
}

pub struct Cell {
    pub ctype: CellType,
    pos: Vec2,
}

impl CellType {
    pub fn color(&self) -> Rgb8 {
        match self {
            CellType::Empty => COLOR_EMPTY.clone(),
            CellType::Wall => COLOR_WALL.clone(),
            CellType::Start => COLOR_START.clone(),
            CellType::Stop => COLOR_STOP.clone(),
        }
    }

    pub fn next(&self) -> CellType {
        match self {
            CellType::Empty => CellType::Wall,
            CellType::Wall => CellType::Start,
            CellType::Start => CellType::Stop,
            CellType::Stop => CellType::Empty,
        }
    }

    pub fn previous(&self) -> CellType {
        match self {
            CellType::Empty => CellType::Stop,
            CellType::Wall => CellType::Empty,
            CellType::Start => CellType::Wall,
            CellType::Stop => CellType::Start,
        }
    }
}

impl Cell {
    pub fn color(&self) -> Rgb8 {
        self.ctype.color()
    }

    pub fn next(&self) -> CellType {
        self.ctype.next()
    }

    pub fn previous(&self) -> CellType {
        self.ctype.previous()
    }
}

/// Structure simulating the terrain (with a board)
pub struct Terrain {
    cell_size: f32,
    board: Vec<Cell>,
    w: usize,
    h: usize,
    x_offset: f32,
    y_offset: f32,
    pub start_stop: [Option<(usize, usize)>; 2],
}

impl Terrain {
    /// Makes a new terrain from width and height
    /// knowing CELL_SIDE
    pub fn new(w: f32, h: f32) -> Terrain {
        let w = (w / CELL_SIDE).ceil() as usize;
        let h = (h / CELL_SIDE).ceil() as usize;

        let mut board: Vec<Cell> = Vec::with_capacity(h * w);

        let x_offset = -((w as f32 - 1.) * CELL_SIDE / 2.);
        let y_offset = -((h as f32 - 1.) * CELL_SIDE / 2.);

        for i in 0..w {
            for j in 0..h {
                let x = x_offset + i as f32 * CELL_SIDE;
                let y = y_offset + j as f32 * CELL_SIDE;

                let new_cell = Cell {
                    ctype: CellType::Empty,
                    pos: Vec2::new(x, y),
                };

                board.push(new_cell);
            }
        }

        println!("Board has {} tiles ({}x{})", w * h, w, h);

        Terrain {
            cell_size: CELL_SIDE,
            board,
            w,
            h,
            x_offset,
            y_offset,
            start_stop: [None; 2],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&Cell> {
        (i < self.w && j < self.h).then(|| &self.board[i * self.h + j])
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Cell> {
        (i < self.w && j < self.h).then(|| &mut self.board[i * self.h + j])
    }

    pub fn get_neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();

        if i > 0 {
            res.push((i - 1, j));
        }
        if j > 0 {
            res.push((i, j - 1));
        }
        if i < self.w - 1 {
            res.push((i + 1, j));
        }
        if j < self.h - 1 {
            res.push((i, j + 1));
        }

        res
    }

    /// Draws the terrain with the color statics
    pub fn draw(&self, draw: &Draw) {
        let rect = Rect::from_w_h(CELL_SIDE, CELL_SIDE);

        for i in 0..self.w {
            for j in 0..self.h {
                draw.rect()
                    .xy(self.get(i, j).unwrap().pos)
                    .wh(rect.wh())
                    .stroke_weight(1.)
                    .stroke(COLOR_BACKGROUND.clone())
                    .color(self.get(i, j).unwrap().color());
            }
        }
    }

    pub fn place(&mut self, position: Vec2, ctype: CellType) {
        let i = position.x - self.x_offset;
        let i = (i / CELL_SIDE).round() as usize;
        let j = position.y - self.y_offset;
        let j = (j / CELL_SIDE).round() as usize;

        if i >= self.w || j  >= self.h {
            return;
        }

        // replace start or stop if needed
        let a = match ctype {
            CellType::Start => Some(0),
            CellType::Stop => Some(1),
            _ => None,
        };

        if let Some(num) = a {
            if let Some((k, l)) = self.start_stop[num] {
                self.get_mut(k, l).unwrap().ctype = CellType::Empty;
            }
            self.start_stop[num] = Some((i, j));
        }

        self.get_mut(i, j).map(|cell| cell.ctype = ctype);
    }
}
