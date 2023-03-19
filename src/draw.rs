use macroquad::{
    color_u8,
    prelude::{vec2, Color, Rect},
    shapes::draw_line,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
    components::{Block, World},
    TILE_SIZE,
};

pub const DARK: Color = color_u8!(49, 47, 40, 255);
pub const LIGHT: Color = color_u8!(218, 216, 209, 255);
pub const DIM: Color = color_u8!(218, 216, 209, 25);

pub trait Drawable {
    fn draw(&self, texture: &Texture2D);
}

impl Drawable for Block {
    fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(
            *texture,
            self.pos.x * TILE_SIZE,
            self.pos.y * TILE_SIZE,
            LIGHT,
            DrawTextureParams {
                dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                source: Some(Rect::new(
                    self.atlas_idx as f32 * TILE_SIZE,
                    0.0,
                    TILE_SIZE,
                    TILE_SIZE,
                )),
                ..Default::default()
            },
        )
    }
}

pub fn draw_grid(world: &World) {
    for x in 0..=(world.size.x as i32) {
        draw_line(
            x as f32 * TILE_SIZE,
            0.0,
            x as f32 * TILE_SIZE,
            world.size.y * TILE_SIZE,
            1.0,
            DIM,
        );
    }
    for y in 0..=(world.size.y as i32) {
        draw_line(
            0.0,
            y as f32 * TILE_SIZE,
            world.size.x as f32 * TILE_SIZE,
            y as f32 * TILE_SIZE,
            1.0,
            DIM,
        );
    }
}
