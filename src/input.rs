use macroquad::{
    prelude::{is_key_down, is_key_released, KeyCode},
    time::get_frame_time,
};

use crate::{components::World, spawner};

pub fn input(w: &mut World) {
    let delta: f32 = 800.0;

    if is_key_down(KeyCode::Down) {
        w.camera.pos.y += delta * get_frame_time();
    } else if is_key_down(KeyCode::Up) {
        w.camera.pos.y -= delta * get_frame_time();
    } else if is_key_down(KeyCode::Right) {
        w.camera.pos.x += delta * get_frame_time();
    } else if is_key_down(KeyCode::Left) {
        w.camera.pos.x -= delta * get_frame_time();
    }

    if is_key_down(KeyCode::Q) {
        w.camera.zoom -= 0.010;
    } else if is_key_down(KeyCode::E) {
        w.camera.zoom += 0.010;
    }

    if is_key_released(KeyCode::R) {
        let _ = spawner::generate_dungeon(&mut w.map);
    }
}
