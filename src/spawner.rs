use std::ops::{Add, Range};

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

pub fn generate_dungeon(map: &mut Map) -> Vec<(Vec2, Tile)> {
    dungeon_1(map)
}

fn generate_rooms(amount: usize, bounds: Vec2) -> Vec<Room> {
    let mut placed_rooms: Vec<Room> = Vec::new();

    while placed_rooms.len() < amount {
        let room_size = ROOM_SIZES[rand::gen_range(0, ROOM_SIZES.len())];
        let mut found_empty_spot = false;
        let mut attemps = 0;
        while !found_empty_spot && attemps < 50 {
            let pos = vec2(
                rand::gen_range(0.0, bounds.x).floor(),
                rand::gen_range(0.0, bounds.y).floor(),
            );
            if pos.x + room_size.x >= GAME_WIDTH || pos.y + room_size.y >= GAME_HEIGHT {
                attemps += 1;
                continue;
            }

            let room = Room::new(pos, room_size);
            found_empty_spot = !placed_rooms.iter().any(|r| r.intersects(&room));

            if found_empty_spot {
                placed_rooms.push(room);
            }
        }
    }

    placed_rooms
}

fn is_room(tile: &Tile) -> bool {
    match tile {
        Tile::SoftFloor | Tile::HardFloor | Tile::SoftWall | Tile::HardWall => true,
        _ => false,
    }
}

fn surrounding_tiles(map: &Map, idx: usize) -> Vec<Option<&Tile>> {
    let adjecent_vecs = vec![
        vec2(-1.0, -1.0), // top left corner
        vec2(0.0, -1.0),  // top center
        vec2(1.0, -1.0),  // top right corner
        vec2(-1.0, 0.0),  // left
        vec2(1.0, 0.0),   // right
        vec2(-1.0, 1.0),  // bottom left corner
        vec2(0.0, 1.0),   // bottom center
        vec2(1.0, 1.0),   // bottom right corner
    ];

    adjecent_vecs
        .iter()
        .map(|v| map.tile_at_pos(map.idx_to_vec2(idx).add(*v)))
        .collect()
}
fn surrounding_idxs(map: &Map, idx: usize) -> Vec<usize> {
    let adjecent_vecs = vec![
        vec2(-1.0, -1.0), // top left corner
        vec2(0.0, -1.0),  // top center
        vec2(1.0, -1.0),  // top right corner
        vec2(-1.0, 0.0),  // left
        vec2(1.0, 0.0),   // right
        vec2(-1.0, 1.0),  // bottom left corner
        vec2(0.0, 1.0),   // bottom center
        vec2(1.0, 1.0),   // bottom right corner
    ];

    adjecent_vecs
        .iter()
        .map(|v| map.idx(map.idx_to_vec2(idx).add(*v)))
        .collect()
}

fn adjecent_idxs(map: &Map, idx: usize) -> Vec<usize> {
    let adjecent_vecs = vec![
        vec2(0.0, -1.0), // top center
        vec2(-1.0, 0.0), // left
        vec2(1.0, 0.0),  // right
        vec2(0.0, 1.0),  // bottom center
    ];

    adjecent_vecs
        .iter()
        .map(|v| {
            let res = map.idx_to_vec2(idx).add(*v);

            if res.x < 0.0 || res.y < 0.0 || res.x >= map.size.x || res.y >= map.size.y {
                return None;
            }

            Some(map.idx(res))
        })
        .filter(|t| t.is_some())
        .map(|t| t.unwrap())
        .collect()
}

fn adjecent_to_room(map: &Map, idx: usize) -> bool {
    let surrounding_tiles = surrounding_tiles(map, idx);

    surrounding_tiles.iter().any(|t| match t {
        Some(t) => is_room(t),
        _ => false,
    })
}

fn is_neighbourless_idx(map: &Map, idx: usize) -> bool {
    !surrounding_tiles(&map, idx).iter().any(|t| match t {
        Some(t) => is_room(t),
        _ => false,
    })
}

fn neighbourless_idxs(map: &Map) -> Vec<usize> {
    let mut starting_points = Vec::new();
    for (idx, tile) in map.tiles.iter().enumerate() {
        if is_room(tile) {
            continue;
        }

        let is_potential_start = is_neighbourless_idx(map, idx);

        if is_potential_start {
            starting_points.push(idx);
        }
    }

    starting_points
}

/**
 * Depth first search to find all tiles that are not connected to any room
 */
fn dfs(map: &mut Map, visited: &mut Vec<usize>, idx: usize) {
    if visited.len() > 100 {
        return;
    }
    visited.push(idx);
    let adjecent = adjecent_idxs(map, idx);

    for adj in adjecent.iter() {
        if adj >= &map.tiles.len() || visited.contains(adj) {
            continue;
        }

        let adjecent_to_any_visited = adjecent_idxs(map, *adj)
            .iter()
            .filter(|i| **i != idx)
            .any(|i| visited.contains(i));
        if !adjecent_to_any_visited && !adjecent_to_room(map, *adj) {
            dfs(map, visited, *adj)
        }
    }
}

fn dungeon_1(map: &mut Map) -> Vec<(Vec2, Tile)> {
    let mut timeline = Vec::new();
    map.tiles = vec![Tile::Dirt; (GAME_WIDTH * GAME_HEIGHT) as usize];

    let rooms = generate_rooms(20, map.size);

    rooms.iter().for_each(|r| {
        let w = r.size.x as usize;
        let h = r.size.y as usize;
        for x in 0..=w {
            for y in 0..=h {
                let tile = match (x, y) {
                    _ if x == 0 || x == w || y == 0 || y == h => Tile::SoftFloor,
                    _ => Tile::SoftFloor,
                };

                let idx = map.idx_xy(r.pos.x as usize + x, r.pos.y as usize + y);
                map.tiles[idx] = tile.clone();
                timeline.push((vec2(x as f32, y as f32), tile));
            }
        }
    });

    let starting_points = neighbourless_idxs(&map);
    // println!("Found starting points {:?}", starting_points);
    let rand_start = starting_points[rand::gen_range(0, starting_points.len() - 1)];
    let mut visited: Vec<usize> = Vec::new();
    dfs(map, &mut visited, rand_start);
    // println!("Visited {:?}", visited);
    visited.iter().for_each(|v| map.tiles[*v] = Tile::SoftFloor);
    timeline
}
