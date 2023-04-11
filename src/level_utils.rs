use macroquad::prelude::*;
use std::ops::Add;

use crate::components::{
    Map, Tile, WALL_BOTTOM_END, WALL_BOTTOM_LEFT_CORNER, WALL_BOTTOM_RIGHT_CORNER, WALL_CROSS,
    WALL_DOWNRIGHT_T, WALL_HOR_LINE, WALL_LEFT_END, WALL_LEFT_LYING_T, WALL_RIGHT_END,
    WALL_RIGHT_LYING_T, WALL_TOP_END, WALL_TOP_LEFT_CORNER, WALL_TOP_RIGHT_CORNER, WALL_UPRIGHT_T,
    WALL_VERT_LINE,
};

pub fn is_floor(tile: &Tile) -> bool {
    match tile {
        Tile::Floor => true,
        _ => false,
    }
}

pub fn is_room(tile: &Tile) -> bool {
    match tile {
        Tile::Floor | Tile::Wall => true,
        _ => false,
    }
}

pub fn get_tile_at_pos(map: &Map, pos: Vec2) -> Option<&Tile> {
    let idx = map.idx(pos);
    map.tiles.get(idx)
}

pub fn surrounding_tiles(map: &Map, idx: usize) -> Vec<Option<&Tile>> {
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
pub fn surrounding_idxs(map: &Map, idx: usize) -> Vec<usize> {
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
        .filter(|u| u < &map.tiles.len())
        .collect()
}

pub fn adjecent_idxs(map: &Map, idx: usize) -> Vec<usize> {
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

pub fn get_wall_atlas_pos(tiles: &Vec<Tile>, surrounding: &Vec<usize>) -> Vec2 {
    let matches = surrounding
        .iter()
        .map(|idx| tiles.get(*idx))
        .map(|t| match t {
            Some(Tile::Wall) => true,
            _ => false,
        })
        .collect::<Vec<bool>>();

    match matches[..] {
        // end pieces
        [_, false, _, true, false, _, false, _] => WALL_RIGHT_END,
        [_, false, _, false, true, _, false, _] => WALL_LEFT_END,
        [_, false, _, false, false, _, true, _] => WALL_TOP_END,
        [_, true, _, false, false, _, false, _] => WALL_BOTTOM_END,
        // connectors
        [_, true, _, true, true, _, true, _] => WALL_CROSS,
        [_, true, _, false, false, _, true, _] => WALL_VERT_LINE,
        [_, false, _, true, true, _, false, _] => WALL_HOR_LINE,
        [_, false, _, true, true, _, true, _] => WALL_UPRIGHT_T,
        [_, true, _, true, true, _, false, _] => WALL_DOWNRIGHT_T,
        [_, true, _, true, false, _, true, _] => WALL_RIGHT_LYING_T,
        [_, true, _, false, true, _, true, _] => WALL_LEFT_LYING_T,
        // corners
        [_, false, _, false, true, _, true, _] => WALL_TOP_LEFT_CORNER,
        [_, false, _, true, false, _, true, _] => WALL_TOP_RIGHT_CORNER,
        [_, true, _, false, true, _, false, _] => WALL_BOTTOM_LEFT_CORNER,
        [_, true, _, true, false, _, false, _] => WALL_BOTTOM_RIGHT_CORNER,
        _ => vec2(8.0, 0.0),
    }
}

pub fn is_adjecent_to_room(map: &Map, idx: usize) -> bool {
    let surrounding_tiles = surrounding_tiles(map, idx);

    surrounding_tiles.iter().any(|t| match t {
        Some(t) => is_room(t),
        _ => false,
    })
}

pub fn is_neighbourless_idx(map: &Map, idx: usize) -> bool {
    !surrounding_tiles(&map, idx).iter().any(|t| match t {
        Some(t) => is_room(t),
        _ => false,
    })
}

pub fn neighbourless_idxs(map: &Map) -> Vec<usize> {
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
