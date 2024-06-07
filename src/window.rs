use minifb::{Key, Window, WindowOptions};
use vek::*;

use crate::{
    cell_types::{Cell, CellType},
    World,
};

pub fn run_window(width: usize, height: usize, world: &mut World) {
    let mut opts = WindowOptions::default();
    opts.scale = minifb::Scale::X2;
    let mut window = Window::new("Test - ESC to exit", width, height, opts).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut cell_type: CellType = CellType::Sand;
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(
        (16600 * world.tick_rate) as u64,
    )));
    let mut tick = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_pressed_keys(&window, &mut cell_type, world);
        handle_mouse(&window, world, cell_type.clone());
        tick += 1;

        if tick >= world.tick_rate {
            tick = 0;
            world.update();
            let buffer = world.draw_scene();
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }
}
pub fn handle_pressed_keys(window: &Window, cell_type: &mut CellType, world: &mut World) {
    window
        .get_keys_pressed(minifb::KeyRepeat::Yes)
        .iter()
        .for_each(|key| match key {
            Key::Key1 => *cell_type = CellType::Sand,
            Key::Key2 => *cell_type = CellType::Stone,
            Key::Key3 => *cell_type = CellType::Wood,
            Key::Key4 => *cell_type = CellType::Fire,
            Key::R => world.cells.clear(),
            _ => (),
        });
}
pub fn handle_mouse(window: &Window, world: &mut World, cell_type: CellType) {
    if window.get_mouse_down(minifb::MouseButton::Left) {
        let position = window.get_mouse_pos(minifb::MouseMode::Clamp);
        if let Some(value) = position {
            let gp = Vec2::new(value.0 as u32, value.1 as u32);
            let clamped = world.snap_pos(gp);
            let cell = Cell::new(cell_type, clamped);
            world.add_cell(cell);
        }
    }
    if window.get_mouse_down(minifb::MouseButton::Right) {
        let position = window.get_mouse_pos(minifb::MouseMode::Clamp);
        if let Some(value) = position {
            let gp = Vec2::new(value.0 as u32, value.1 as u32);
            let clamped = world.snap_pos(gp);
            world.destroy_cell(clamped);
        }
    }
}
