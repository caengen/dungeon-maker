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

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let blocks_texture: Texture2D = load_texture("assets/blocks.png").await.unwrap();
    let mut blocks = Vec::new();
    let mut spawned = spawn_blocks_in_area(vec2(5.0, 5.0), vec2(10.0, 10.0), &blocks_texture);
    blocks.append(&mut spawned);
    loop {
        clear_background(DARK);

        blocks.iter().for_each(|b| b.draw());

        next_frame().await
    }
}
