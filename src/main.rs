use macroquad::{prelude::*, window};

mod components;
use components::*;
use draw::*;
mod draw;
use input::*;
mod input;
mod level;
mod level_utils;

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
    let dungeon_texture: Texture2D = load_texture("assets/Dungeon.png").await.unwrap();

    let mut world = World::new(GAME_WIDTH, GAME_HEIGHT);

    let _steps = level::generate_dungeon(&mut world.map);

    loop {
        clear_background(DARK);
        let camera = Camera2D::from_display_rect(macroquad::math::Rect {
            x: world.camera.pos.x,
            y: world.camera.pos.y,
            w: window::screen_width() * world.camera.zoom,
            h: window::screen_height() * world.camera.zoom,
        });
        set_camera(&camera);

        input(&mut world);
        // timeline.update(&world);

        draw::draw_grid(&world);
        // timeline.draw(&blocks_texture);
        world.map.draw(&dungeon_texture);

        next_frame().await
    }
}
