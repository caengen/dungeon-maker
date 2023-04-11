use std::ops::Range;

use macroquad::{prelude::*, texture::Texture2D};

use crate::{
    components::{AtlasTile, Block, Map, Rect, Room, Size, Tile, ROOM_SIZES},
    level_utils::{
        adjecent_idxs, get_tile_at_pos, get_wall_atlas_pos, is_adjecent_to_room, is_floor, is_room,
        neighbourless_idxs, surrounding_idxs,
    },
    GAME_HEIGHT, GAME_WIDTH,
};

const ROOM_GENERATION_ATTEMPTS: i32 = 50;
const CORRIDOR_MAX_LENGTH: usize = 15;

pub fn generate_dungeon(map: &mut Map) -> Vec<(Vec2, Tile)> {
    dungeon_1(map)
}

/**
 * Returns a Vec of size n of various Room structs within the given bounds.
 */
fn generate_rooms(amount: usize, bounds: Vec2) -> Vec<Room> {
    let mut placed_rooms: Vec<Room> = Vec::new();

    while placed_rooms.len() < amount {
        let room_size = ROOM_SIZES[rand::gen_range(0, ROOM_SIZES.len())];
        let mut found_empty_spot = false;
        let mut attemps = 0;
        while !found_empty_spot && attemps < ROOM_GENERATION_ATTEMPTS {
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

/**
 * Takes a room and a range of x and y values. For each x and y value it checks
 *  if the tile at that the door_pos is "empty" and the tile at the other_room_pos
 * is floor. If so it stores the index of the door_pos tile. Finally it returns at
 * random one of the stored indices. If no doors were found it returns None.
 */
fn generate_doors(
    map: &mut Map,
    room: &Room,
    x_max: Range<i32>,
    y_max: Range<i32>,
    door_pos: Vec2,
    other_room_pos: Vec2,
) -> Option<usize> {
    let mut group = Vec::new();
    for x in x_max {
        for y in y_max.clone() {
            let door_lookup_pos = vec2(
                room.pos.x + x as f32 + door_pos.x,
                room.pos.y + y as f32 + door_pos.y,
            );
            let maybe_door_tile = get_tile_at_pos(map, door_lookup_pos);
            let maybe_other_room_tile = get_tile_at_pos(
                map,
                vec2(
                    room.pos.x + x as f32 + other_room_pos.x,
                    room.pos.y + y as f32 + other_room_pos.y,
                ),
            );
            if maybe_door_tile.is_some() && maybe_other_room_tile.is_some() {
                let tile_space = maybe_door_tile.unwrap();
                let tile_maybe_connection = maybe_other_room_tile.unwrap();
                if !is_room(tile_space) && is_room(tile_maybe_connection) {
                    group.push(map.idx(door_lookup_pos));
                }
            }
        }
    }

    if group.len() > 0 {
        let chosen = rand::gen_range(0, group.len());
        map.tiles[group[chosen]] = if rand::gen_range(0.0, 1.0) > 0.5 {
            Tile::Door
        } else {
            Tile::Floor
        };
        Some(group[chosen])
    } else {
        None
    }
}

/**
 * Depth first search to find all tiles that are not connected to any room
 */
fn dfs(map: &mut Map, visited: &mut Vec<usize>, idx: usize) {
    if visited.len() > CORRIDOR_MAX_LENGTH {
        return;
    }
    let adjecent = adjecent_idxs(map, idx);
    if is_room(&map.tiles[idx]) || is_adjecent_to_room(map, idx) {
        return;
    } else {
        visited.push(idx);
    }

    for adj in adjecent.iter() {
        if adj >= &map.tiles.len() || visited.contains(adj) {
            continue;
        }

        let adjecent_to_any_visited = adjecent_idxs(map, *adj)
            .iter()
            .filter(|i| **i != idx)
            .any(|i| visited.contains(i));
        if !adjecent_to_any_visited && !is_adjecent_to_room(map, *adj) {
            dfs(map, visited, *adj)
        }
    }
}

/**
 * Generate a sparse dungeon with rooms and corridors
 */
fn dungeon_1(map: &mut Map) -> Vec<(Vec2, Tile)> {
    let mut timeline = Vec::new();
    map.tiles = vec![Tile::Dirt; (GAME_WIDTH * GAME_HEIGHT) as usize];

    // place rooms
    let rooms = generate_rooms(rand::gen_range(8, 12), map.size);
    rooms.iter().for_each(|r| {
        let w = r.size.x as usize;
        let h = r.size.y as usize;
        for x in 0..=w {
            for y in 0..=h {
                let tile = match (x, y) {
                    _ if x == 0 || x == w || y == 0 || y == h => Tile::Floor,
                    _ => Tile::Floor,
                };

                let idx = map.idx_xy(r.pos.x as usize + x, r.pos.y as usize + y);
                map.tiles[idx] = tile.clone();
                timeline.push((vec2(x as f32, y as f32), tile));
            }
        }
    });

    // place corridors
    let starting_points = neighbourless_idxs(&map);
    let mut corridors = Vec::new();
    for start in starting_points.iter() {
        let mut visited: Vec<usize> = Vec::new();
        dfs(map, &mut visited, *start);
        // println!("Visited {:?}", visited);
        visited.iter().for_each(|v| {
            map.tiles[*v] = Tile::Floor;
            timeline.push((map.idx_to_vec2(*start), Tile::Floor));
        });
        corridors.push(visited);
    }

    let mut doors = Vec::new();
    // group possible doors by room edge and pick one for each edge of each room
    rooms.iter().for_each(|r| {
        let w = r.size.x as i32;
        let h = r.size.y as i32;

        // traverse bottom
        doors.push(generate_doors(
            map,
            r,
            0..w,
            0..1,
            vec2(0.0, -1.0),
            vec2(0.0, -2.0),
        ));
        // traverse top
        doors.push(generate_doors(
            map,
            r,
            0..w,
            h..(h + 1),
            vec2(0.0, 1.0),
            vec2(0.0, 2.0),
        ));
        // traverse left
        doors.push(generate_doors(
            map,
            r,
            0..1,
            0..h,
            vec2(-1.0, 0.0),
            vec2(-2.0, 0.0),
        ));
        // traverse right
        doors.push(generate_doors(
            map,
            r,
            w..(w + 1),
            0..h,
            vec2(1.0, 0.0),
            vec2(2.0, 0.0),
        ));
    });
    let doors = doors
        .iter()
        .filter(|d| d.is_some())
        .map(|d| d.unwrap())
        .collect::<Vec<usize>>();

    // remove dead ends and non-connected corridors
    for corridor in corridors.iter() {
        let adjacents = corridor
            .iter()
            .map(|c| adjecent_idxs(map, *c))
            .flatten()
            .collect::<Vec<usize>>();
        if adjacents.iter().any(|a| doors.contains(&a)) {
            continue;
        }

        for c in corridor.iter() {
            map.tiles[*c] = Tile::Dirt;
        }
    }

    // add walls
    let mut walls = Vec::new();
    for (idx, tile) in map.tiles.iter().enumerate() {
        if !is_floor(tile) {
            continue;
        }

        let surrounding = surrounding_idxs(&map, idx);
        for s in surrounding.iter() {
            if &map.tiles[*s] == &Tile::Dirt {
                walls.push(*s);
            }
        }
    }

    walls.iter().for_each(|w| {
        map.tiles[*w] = Tile::Wall;
        timeline.push((map.idx_to_vec2(*w), Tile::Wall));
    });

    let mut textures = vec![AtlasTile::from(vec2(7.0, 0.0)); (GAME_WIDTH * GAME_HEIGHT) as usize];
    map.tiles.iter().enumerate().for_each(|(idx, t)| match t {
        Tile::Wall => {
            let surrounding = surrounding_idxs(&map, idx);
            let atlas_pos = get_wall_atlas_pos(&map.tiles, &surrounding);
            textures[idx] = AtlasTile::from(atlas_pos);
        }
        Tile::Floor => textures[idx] = AtlasTile::from(vec2(8.0, 8.0)),
        Tile::Door => textures[idx] = AtlasTile::from(vec2(6.0, 2.0)),
        Tile::Dirt => textures[idx] = AtlasTile::from(vec2(9.0, 6.0)),
        _ => {}
    });
    map.draw_tiles = textures;

    timeline
}
