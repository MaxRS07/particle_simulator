mod cell_types;
mod game;
mod utils;
mod window;

use cell_types::Cell;
use game::*;
use vek::Vec2;
use window::*;

fn main() {
    let width: usize = 256;
    let height: usize = 256;
    let mut world = World::new(width, height, 8, 1, 0, 0);
    run_window(width, height, &mut world);
}
