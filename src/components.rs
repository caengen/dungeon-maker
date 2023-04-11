use crate::{draw::Drawable, GAME_HEIGHT, GAME_WIDTH};
use derive_more::From;
use macroquad::{
    prelude::{vec2, Vec2},
    texture::{draw_texture_ex, Texture2D},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

pub static ROOM_SIZES: [Vec2; 2] = [vec2(5.0, 5.0), vec2(5.0, 7.0)];

#[derive(PartialEq)]
pub enum WallMaterial {
    Stone,
    Brick,
}
#[derive(PartialEq)]
pub enum WallType {
    Cross,
    HorinzontalCenter,
    HorisonalLeftEnd,
    HorisonalRightEnd,
    VerticalCenter,
    ReverseL,
    L,
    T,
    ReverseT,
}

#[derive(PartialEq)]
pub struct Wall {
    pub material: WallMaterial,
    pub wall_type: WallType,
    pub atlas_pos: Vec2,
}

pub trait Updateable {
    fn update(&mut self, world: &World);
}

pub struct CameraControl {
    pub pos: Vec2,
    pub zoom: Vec2,
}
pub struct World {
    pub size: Vec2,
    pub camera: CameraControl,
    pub map: Map,
}

impl World {
    pub fn new(w: f32, h: f32) -> World {
        World {
            size: vec2(w, h),
            camera: CameraControl {
                pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
                zoom: vec2(0.0025, 0.0025),
            },
            map: Map {
                size: vec2(GAME_WIDTH, GAME_HEIGHT),
                tiles: vec![Tile::Dirt; (GAME_WIDTH * GAME_HEIGHT) as usize],
                textures: vec![Texture::from(vec2(7.0, 0.0)); (GAME_WIDTH * GAME_HEIGHT) as usize],
            },
        }
    }
}

pub struct Block {
    pub pos: Vec2,
    pub atlas_idx: i32,
}

pub struct Timer {
    target: f32,
    current: f32,
}

impl Timer {
    pub fn new(target_in_seconds: f32) -> Timer {
        Timer {
            target: target_in_seconds,
            current: 0.0,
        }
    }
    pub fn tick(&mut self, delta: f32) {
        self.current += delta;
    }

    pub fn is_finished(&self) -> bool {
        self.current >= self.target
    }

    pub fn roll_over(&mut self) {
        self.current = self.current - self.target;
    }
}

pub enum TimelineState {
    Paused,
    Running,
    Finished,
}
pub struct Timeline<'a> {
    pub timer: Timer,
    pub state: TimelineState,
    pub cursor: usize,
    pub draws: Vec<&'a dyn Drawable>,
}

impl Timeline<'_> {
    pub fn new() -> Timeline<'static> {
        Timeline {
            timer: Timer::new(1.0),
            state: TimelineState::Paused,
            cursor: 0,
            draws: Vec::new(),
        }
    }

    pub fn from_drawables<T>(value: &mut Vec<T>, transition_speed: f32) -> Timeline
    where
        T: Drawable,
    {
        let mut timeline = Timeline {
            timer: Timer::new(transition_speed),
            state: TimelineState::Paused,
            cursor: 0,
            draws: Vec::new(),
        };

        for d in value.iter_mut() {
            timeline.draws.push(d);
        }

        timeline
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.state = TimelineState::Paused;
    }

    pub fn start(&mut self) {
        self.state = TimelineState::Running;
    }
}

impl Drawable for Timeline<'_> {
    fn draw(&self, texture: &Texture2D) {
        match self.state {
            TimelineState::Running => self
                .draws
                .iter()
                .take(self.cursor)
                .for_each(|d| d.draw(texture)),
            TimelineState::Finished => self.draws.iter().for_each(|d| d.draw(texture)),
            _ => {}
        }
    }
}

impl Updateable for Timeline<'_> {
    fn update(&mut self, _: &World) {
        match self.state {
            TimelineState::Running => {
                self.timer.tick(get_frame_time());

                if self.timer.is_finished() {
                    self.cursor = usize::min(self.cursor + 1, self.draws.len());
                    if self.cursor == self.draws.len() {
                        self.state = TimelineState::Finished;
                    } else {
                        self.timer.roll_over();
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    Dirt,
}

#[derive(Clone, PartialEq, From)]
pub struct Texture(pub Vec2);

pub struct Map {
    pub size: Vec2,
    pub tiles: Vec<Tile>,
    pub textures: Vec<Texture>,
}

impl Map {
    pub fn idx(&self, pos: Vec2) -> usize {
        (pos.y * self.size.y + pos.x) as usize
    }

    pub fn idx_xy(&self, x: usize, y: usize) -> usize {
        y * self.size.y as usize + x
    }

    pub fn idx_to_vec2(&self, idx: usize) -> Vec2 {
        Vec2 {
            x: idx as f32 % self.size.x,
            y: (idx as f32 / self.size.x).floor(),
        }
    }

    pub fn tile_at_pos(&self, pos: Vec2) -> Option<&Tile> {
        let idx = self.idx(pos);
        if pos.x < 0.0 || pos.y < 0.0 || idx >= self.tiles.len() {
            return None;
        }

        Some(&self.tiles[idx])
    }
}

pub struct Room {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Room {
    pub fn new(pos: Vec2, size: Vec2) -> Room {
        Room { pos, size }
    }
}

pub trait Position {
    fn pos(&self) -> Vec2;
}

impl Position for Room {
    fn pos(&self) -> Vec2 {
        self.pos
    }
}

pub trait Size {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
}

impl Size for Room {
    fn width(&self) -> f32 {
        self.size.x
    }
    fn height(&self) -> f32 {
        self.size.y
    }
}

pub trait Rect {
    fn intersects<T: Rect + Position + Size>(&self, other: &T) -> bool;
}

impl Rect for Room {
    fn intersects<T: Rect + Position + Size>(&self, other: &T) -> bool {
        let left = f32::max(self.pos.x, other.pos().x);
        let right = f32::min(self.pos.x + self.size.x, other.pos().x + other.width());
        let top = f32::max(self.pos.y, other.pos().y);
        let bottom = f32::min(self.pos.y + self.size.y, other.pos().y + other.height());

        left < right && top < bottom
    }
}
