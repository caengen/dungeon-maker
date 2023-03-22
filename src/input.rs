use macroquad::{
    prelude::{is_key_down, KeyCode},
    time::get_frame_time,
};

use crate::components::World;

pub fn input(w: &mut World) {
    let delta: f32 = 200.0;

    if is_key_down(KeyCode::Down) {
        w.camera.pos.y -= delta * get_frame_time();
    } else if is_key_down(KeyCode::Up) {
        w.camera.pos.y += delta * get_frame_time();
    } else if is_key_down(KeyCode::Right) {
        w.camera.pos.x += delta * get_frame_time();
    } else if is_key_down(KeyCode::Left) {
        w.camera.pos.x -= delta * get_frame_time();
    }

    if is_key_down(KeyCode::Q) {
        w.camera.zoom.y -= 0.00010;
        w.camera.zoom.x -= 0.00010;
    } else if is_key_down(KeyCode::E) {
        w.camera.zoom.y += 0.00010;
        w.camera.zoom.x += 0.00010;
    }
}
