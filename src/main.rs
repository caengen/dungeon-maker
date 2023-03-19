use macroquad::{prelude::*, window};

mod components;
use components::*;
use draw::*;
mod draw;
use input::*;
mod input;
mod spawner;

pub const TILE_SIZE: f32 = 16.0;
pub const GAME_WIDTH: f32 = 32.0;
pub const GAME_HEIGHT: f32 = 32.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Dungeonmaker".to_owned(),
        window_width: (GAME_WIDTH * TILE_SIZE) as i32,
        window_height: (GAME_HEIGHT * TILE_SIZE) as i32,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new(64.0, 64.0);

    rand::srand(macroquad::miniquad::date::now() as u64);
    let blocks_texture: Texture2D = load_texture("assets/blocks.png").await.unwrap();
    let dungeon = spawner::test_dungeon();
    loop {
        clear_background(DARK);
        input(&mut world);

        set_camera(&Camera2D {
            zoom: world.camera.zoom,
            target: world.camera.pos,
            ..Default::default()
        });

        draw::draw_grid(&world);
        dungeon.iter().for_each(|b| b.draw(&blocks_texture));

        next_frame().await
    }
}
