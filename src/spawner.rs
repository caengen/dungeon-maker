use macroquad::{prelude::*, texture::Texture2D};

use crate::components::Block;

fn spawn_blocks_in_area(start: Vec2, end: Vec2) -> Vec<Block> {
    let mut blocks = Vec::new();
    for x in (start.x as i32)..=(end.x as i32) {
        for y in (start.y as i32)..=(end.y as i32) {
            let block = Block {
                atlas_idx: rand::gen_range(0, 6),
                pos: vec2(x as f32, y as f32),
            };
            blocks.push(block);
        }
    }

    blocks
}

pub fn test_dungeon() -> Vec<Block> {
    let mut blocks = Vec::new();
    blocks.append(&mut spawn_blocks_in_area(
        vec2(15.0, 15.0),
        vec2(16.0, 16.0),
    ));
    blocks.append(&mut spawn_blocks_in_area(vec2(0.0, 5.0), vec2(0.0, 10.0)));
    blocks.append(&mut spawn_blocks_in_area(vec2(0.0, 5.0), vec2(5.0, 5.0)));
    blocks.append(&mut spawn_blocks_in_area(vec2(0.0, 10.0), vec2(5.0, 10.0)));

    blocks
}
