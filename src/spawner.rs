use std::ops::Range;

use macroquad::{prelude::*, texture::Texture2D};

use crate::{
    components::{Block, Map, Rect, Room, Size, Tile, ROOM_SIZES},
    GAME_HEIGHT, GAME_WIDTH,
};

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

pub fn generate_dungeon(map: &mut Map) -> Vec<(Vec2, Tile)> {
    dungeon_1(map)
}

pub fn dungeon_1(map: &mut Map) -> Vec<(Vec2, Tile)> {
    let mut timeline = Vec::new();
    let rooms_amount = 15;
    let mut placed_rooms: Vec<Room> = Vec::new();
    map.tiles = vec![Tile::Dirt; (GAME_WIDTH * GAME_HEIGHT) as usize];

    for i in 0..rooms_amount {
        let room_size = ROOM_SIZES[rand::gen_range(0, ROOM_SIZES.len())];
        let mut found_empty_spot = false;
        while !found_empty_spot {
            let pos = vec2(
                rand::gen_range(0.0, map.size.x).floor(),
                rand::gen_range(0.0, map.size.y).floor(),
            );
            if pos.x + room_size.x >= GAME_WIDTH || pos.y + room_size.y >= GAME_HEIGHT {
                continue;
            }

            let room = Room::new(pos, room_size);
            found_empty_spot = !placed_rooms.iter().any(|r| r.intersects(&room));

            if found_empty_spot {
                placed_rooms.push(room);
                println!("Placed room at x:{}, y:{}", pos.x, pos.y);
            }
        }
    }

    placed_rooms.iter().for_each(|r| {
        let w = r.size.x as usize;
        let h = r.size.y as usize;
        for x in 0..=w {
            for y in 0..=h {
                let tile = match (x, y) {
                    _ if x == 0 || x == w || y == 0 || y == h => Tile::SoftWall,
                    _ => Tile::SoftFloor,
                };

                let idx = map.idx_xy(r.pos.x as usize + x, r.pos.y as usize + y);
                map.tiles[idx] = tile.clone();
                timeline.push((vec2(x as f32, y as f32), tile));
            }
        }
    });

    timeline
}
