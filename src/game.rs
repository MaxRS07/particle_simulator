use std::cell;

use vek::*;

use crate::cell_types::*;
#[derive(Clone)]
pub struct World {
    pub tick_rate: u32,
    pub tick_speed: u32,
    pub gravity: u32,
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub cell_size: u32,
}
impl World {
    pub fn new(
        width: usize,
        height: usize,
        cell_size: u32,
        tick_rate: u32,
        tick_speed: u32,
        gravity: u32,
    ) -> World {
        return World {
            width,
            height,
            cells: vec![],
            cell_size,
            tick_rate,
            tick_speed,
            gravity,
        };
    }
    pub fn add_cell(&mut self, mut cell: Cell) -> bool {
        let snapped = self.snap_pos(cell.pos);
        self.destroy_cell(snapped);
        cell.pos = snapped;
        self.cells.push(cell);
        return true;
    }
    pub fn destroy_cell(&mut self, pos: Vec2<u32>) {
        for i in 0..self.cells.len() {
            if self.cells[i].pos == pos {
                self.cells.remove(i);
                return;
            }
        }
    }
    pub fn cell_type(self, pos: Vec2<u32>) -> CellType {
        let value = self
            .cells
            .iter()
            .find(|c| self.snap_pos(c.pos) == self.snap_pos(pos));
        match value {
            Some(x) => x.cell_type,
            None => CellType::Empty,
        }
    }
    pub fn get_cell(&mut self, pos: Vec2<u32>) -> Option<&mut Cell> {
        let pos = self.clone().snap_pos(pos);
        for cell in &mut self.cells {
            if cell.pos == pos {
                return Some(cell);
            }
        }
        None
    }
    pub fn update(&mut self) {
        for i in 0..self.cells.len() {
            let clone = self.clone();
            self.cells[i].update(&clone);
        }
        let clone = self.clone();
        for cell in &clone.cells {
            if cell.cell_type == CellType::Empty {
                self.destroy_cell(cell.pos)
            }
        }
    }

    pub fn draw_scene(&self) -> Vec<u32> {
        let mut buffer = vec![0; self.width * self.height];
        for cell in &self.cells {
            let ypos = cell.pos.y as usize * self.width;
            let xpos = cell.pos.x as usize;
            for i in 0..self.cell_size {
                for j in 0..self.cell_size {
                    buffer[ypos + i as usize * self.width + xpos + j as usize] = cell.get_color()
                }
            }
        }
        return buffer;
    }
    pub fn snap_pos(&self, mut pos: Vec2<u32>) -> Vec2<u32> {
        pos.x = pos.x / self.cell_size * self.cell_size;
        pos.y = pos.y / self.cell_size * self.cell_size;

        return pos;
    }
    pub fn out_of_bounds(&self, mut pos: Vec2<u32>) -> bool {
        pos = self.snap_pos(pos);
        return pos.y >= self.height as u32 || pos.x >= self.width as u32;
    }
    pub fn has_tile(&self, pos: Vec2<u32>) -> bool {
        return self
            .cells
            .iter()
            .any(|c| self.snap_pos(c.pos) == self.snap_pos(pos))
            || self.out_of_bounds(pos);
    }
    pub fn has_tile_solid(&self, pos: Vec2<u32>) -> bool {
        return self
            .cells
            .iter()
            .any(|c| self.snap_pos(c.pos) == self.snap_pos(pos) && c.cell_type.solid_tile())
            || self.out_of_bounds(pos);
    }
}
