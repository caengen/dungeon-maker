use macroquad::{
    prelude::{vec2, Vec2},
    texture::{draw_texture_ex, Texture2D},
    window::{screen_height, screen_width},
};

pub struct CameraControl {
    pub pos: Vec2,
    pub zoom: Vec2,
}
pub struct World {
    pub size: Vec2,
    pub camera: CameraControl,
}

pub struct Block {
    pub pos: Vec2,
    pub atlas_idx: i32,
}

impl World {
    pub fn new(w: f32, h: f32) -> World {
        World {
            size: vec2(w, h),
            camera: CameraControl {
                pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
                zoom: vec2(0.0025, 0.0025),
            },
        }
    }
}
