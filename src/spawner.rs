use std::ops::Range;

use macroquad::{prelude::*, texture::Texture2D};

use crate::{components::Block, GAME_HEIGHT, GAME_WIDTH};

fn spawn_blocks_in_area(start: Vec2, end: Vec2) -> Vec<Block> {
    spawn_tile_in_area(start, end, 0..6)
}

fn spawn_floor_in_area(start: Vec2, end: Vec2) -> Vec<Block> {
    spawn_tile_in_area(start, end, 6..7)
}

fn spawn_tile_in_area(start: Vec2, end: Vec2, atlas_idx_range: Range<i32>) -> Vec<Block> {
    let mut blocks = Vec::new();
    for x in (start.x as i32)..=(end.x as i32) {
        for y in (start.y as i32)..=(end.y as i32) {
            let block = Block {
                atlas_idx: rand::gen_range(atlas_idx_range.start, atlas_idx_range.end),
                pos: vec2(x as f32, y as f32),
            };
            blocks.push(block);
        }
    }

    blocks
}

fn spawn_room(pos: Vec2, width: f32, height: f32) -> Vec<Block> {
    let mut blocks = Vec::new();
    blocks.append(&mut spawn_blocks_in_area(
        vec2(pos.x, pos.y),
        vec2(pos.x + width, pos.y),
    ));
    blocks.append(&mut spawn_blocks_in_area(
        vec2(pos.x, pos.y + 1.0),
        vec2(pos.x, pos.y + height - 1.0),
    ));
    blocks.append(&mut spawn_blocks_in_area(
        vec2(pos.x, pos.y + height),
        vec2(pos.x + width, pos.y + height),
    ));
    blocks.append(&mut spawn_blocks_in_area(
        vec2(pos.x + width, pos.y + 1.0),
        vec2(pos.x + width, pos.y + height - 1.0),
    ));

    blocks
}

pub fn simple_dungeon() -> Vec<Block> {
    let rooms_target = rand::gen_range(5, 8);
    let mut rooms_placed = 0;
    let mut blocks = Vec::new();

    while rooms_placed < rooms_target {
        let size = (rand::gen_range(7, 13) as f32, rand::gen_range(7, 13) as f32);
        let mut pos = (
            rand::gen_range(0, GAME_WIDTH as i32) as f32,
            rand::gen_range(0, GAME_HEIGHT as i32) as f32,
        );
        if pos.0 + size.0 > GAME_WIDTH {
            pos.0 = GAME_WIDTH - size.0;
        }
        if pos.1 + size.1 > GAME_HEIGHT {
            pos.1 = GAME_HEIGHT - size.1;
        }

        blocks.append(&mut spawn_room(vec2(pos.0, pos.1), size.0, size.1));
        blocks.append(&mut spawn_floor_in_area(
            vec2(pos.0 + 1.0, pos.1 + 1.0),
            vec2(pos.0 + size.0 - 1.0, pos.1 + size.1 - 1.0),
        ));

        rooms_placed += 1;
    }

    blocks
}
