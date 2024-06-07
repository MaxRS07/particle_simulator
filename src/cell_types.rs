use std::cell;

use crate::{utils::*, World};
use minifb::clamp;
use rand::random;
use vek::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    Empty,
    Stone,
    Wood,
    Fire,
    Water,
    Sand,
}
impl CellType {
    pub fn can_burn(&self) -> bool {
        match self {
            CellType::Wood => true,
            _ => false,
        }
    }
    pub fn solid_tile(&self) -> bool {
        match self {
            CellType::Empty => false,
            CellType::Fire => false,
            CellType::Water => false,
            _ => true,
        }
    }
}
#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub pos: Vec2<u32>,
    pub vel: Vec2<i32>,
}
impl Cell {
    pub fn new(cell_type: CellType, pos: Vec2<u32>) -> Self {
        let vel = Vec2::new(0, 0);
        return Cell {
            cell_type,
            pos,
            vel,
        };
    }
    pub fn get_color(&self) -> u32 {
        match &self.cell_type {
            CellType::Stone => enccol(136, 140, 141),
            CellType::Wood => enccol(92, 64, 51),
            CellType::Water => enccol(0, 0, 0),
            CellType::Sand => enccol(194, 178, 128),
            CellType::Fire => enccol(255, 154, 0),
            _ => enccol(0, 0, 0),
        }
    }
    pub fn update(&mut self, world: &World) {
        if (random::<u8>() == 0) {
            self.random_tick(world)
        }
        let isize = world.cell_size as i32;
        let down = Vec2::unit_y() * world.cell_size;
        let right = Vec2::unit_x() * world.cell_size;
        match &self.cell_type {
            CellType::Stone => {}
            CellType::Sand => {
                if !world.has_tile(self.pos + down) {
                    self.vel = Vec2::new(0, isize);
                } else if self.pos.x as i32 - 1 > 0
                    && !world.has_tile_solid(self.pos + down - right)
                    && !world.has_tile_solid(self.pos - right)
                {
                    self.vel.x = -isize;
                    self.vel.y = isize;
                } else if self.pos.x + 1 < world.width as u32
                    && !world.has_tile_solid(self.pos + down + right)
                    && !world.has_tile_solid(self.pos + right)
                {
                    self.vel.x = isize;
                    self.vel.y = isize;
                } else {
                    self.vel = Vec2::zero();
                }
                self.shift(self.vel.x, self.vel.y)
            }
            CellType::Wood => {}
            _ => {}
        }
    }
    pub fn random_tick(&mut self, world: &World) {
        if self.cell_type.can_burn() {
            for i in 0..3 {
                for j in 0..3 {
                    let deref = world.clone();
                    let dx = (i as i32 - 1) * world.cell_size as i32;
                    let dy = (j as i32 - 1) * world.cell_size as i32;
                    let nx = (self.pos.x as i32 + dx) as u32;
                    let ny = (self.pos.y as i32 + dy) as u32;
                    let pos = Vec2::new(nx, ny);
                    if nx > world.width as u32 || ny > world.height as u32 || pos == self.pos {
                        continue;
                    }
                    if deref.cell_type(pos) == CellType::Fire {
                        self.cell_type = CellType::Fire;
                    }
                }
            }
        }
        match self.cell_type {
            CellType::Fire => {
                self.cell_type = CellType::Empty;
            }
            _ => (),
        }
    }
    pub fn shift(&mut self, x: i32, y: i32) {
        self.pos.x = (self.pos.x as i32 + x) as u32;
        self.pos.y = (self.pos.y as i32 + y) as u32;
    }
}
