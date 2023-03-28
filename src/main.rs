use macroquad::{prelude::*, window};

mod components;
use components::*;
use draw::*;
mod draw;
use input::*;
mod input;
mod spawner;

pub const TILE_SIZE: f32 = 16.0;
pub const GAME_WIDTH: f32 = 64.0;
pub const GAME_HEIGHT: f32 = 64.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Dungeonmaker".to_owned(),
        window_width: (32.0 * TILE_SIZE) as i32,
        window_height: (32.0 * TILE_SIZE) as i32,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let blocks_texture: Texture2D = load_texture("assets/blocks.png").await.unwrap();

    let mut world = World::new(GAME_WIDTH, GAME_HEIGHT);
    // let mut dungeon = spawner::simple_dungeon();

    let _steps = spawner::generate_dungeon(&mut world.map);

    // let mut timeline = Timeline::from_tiles_steps()
    // let mut timeline = Timeline::from_drawables(&mut dungeon, 0.1);
    // timeline.start();

    loop {
        clear_background(DARK);
        set_camera(&Camera2D {
            zoom: world.camera.zoom,
            target: world.camera.pos,
            ..Default::default()
        });

        input(&mut world);
        // timeline.update(&world);

        draw::draw_grid(&world);
        // dungeon.iter().for_each(|b| b.draw(&blocks_texture));
        // timeline.draw(&blocks_texture);
        world.map.draw(&blocks_texture);

        next_frame().await
    }
}
