use macroquad::{prelude::*, window};

pub const DARK: Color = color_u8!(49, 47, 40, 255);
pub const LIGHT: Color = color_u8!(218, 216, 209, 255);

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

trait Drawable {
    fn draw(&self);
}

struct Block<'a> {
    pos: Vec2,
    atlas_idx: i32,
    texture: &'a Texture2D,
}

impl Drawable for Block<'_> {
    fn draw(&self) {
        draw_texture_ex(
            *self.texture,
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

fn spawn_blocks_in_area(start: Vec2, end: Vec2, texture: &Texture2D) -> Vec<Block> {
    let mut blocks = Vec::new();
    for x in (start.x as i32)..=(end.x as i32) {
        for y in (start.y as i32)..=(end.y as i32) {
            let block = Block {
                atlas_idx: rand::gen_range(0, 6),
                pos: vec2(x as f32, y as f32),
                texture: &texture,
            };
            blocks.push(block);
        }
    }

    blocks
}

pub struct CameraControl {
    pub pos: Vec2,
    pub zoom: Vec2,
}
pub struct GameState {
    pub camera: CameraControl,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            camera: CameraControl {
                pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
                zoom: vec2(0.0025, 0.0025),
            },
        }
    }
}

pub fn keyboard_listeners(gs: &mut GameState) {
    let delta: f32 = if is_key_down(KeyCode::LeftShift) {
        200.0
    } else {
        100.0
    };

    if is_key_down(KeyCode::Down) {
        gs.camera.pos.y -= delta * get_frame_time();
    } else if is_key_down(KeyCode::Up) {
        gs.camera.pos.y += delta * get_frame_time();
    } else if is_key_down(KeyCode::Right) {
        gs.camera.pos.x += delta * get_frame_time();
    } else if is_key_down(KeyCode::Left) {
        gs.camera.pos.x -= delta * get_frame_time();
    }

    if is_key_down(KeyCode::Q) {
        gs.camera.zoom.y -= 0.010 * get_frame_time();
        gs.camera.zoom.x -= 0.010 * get_frame_time();
    } else if is_key_down(KeyCode::E) {
        gs.camera.zoom.y += 0.010 * get_frame_time();
        gs.camera.zoom.x += 0.010 * get_frame_time();
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gs = GameState::new();

    rand::srand(macroquad::miniquad::date::now() as u64);
    let blocks_texture: Texture2D = load_texture("assets/blocks.png").await.unwrap();
    let mut blocks = Vec::new();
    blocks.append(&mut spawn_blocks_in_area(
        vec2(15.0, 15.0),
        vec2(16.0, 16.0),
        &blocks_texture,
    ));
    blocks.append(&mut spawn_blocks_in_area(
        vec2(0.0, 5.0),
        vec2(0.0, 10.0),
        &blocks_texture,
    ));
    blocks.append(&mut spawn_blocks_in_area(
        vec2(0.0, 5.0),
        vec2(5.0, 5.0),
        &blocks_texture,
    ));
    blocks.append(&mut spawn_blocks_in_area(
        vec2(0.0, 10.0),
        vec2(5.0, 10.0),
        &blocks_texture,
    ));
    loop {
        clear_background(DARK);
        keyboard_listeners(&mut gs);

        set_camera(&Camera2D {
            zoom: gs.camera.zoom,
            target: gs.camera.pos,
            ..Default::default()
        });
        // set_default_camera();

        blocks.iter().for_each(|b| b.draw());

        next_frame().await
    }
}
