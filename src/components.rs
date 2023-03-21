use macroquad::{
    prelude::{vec2, Vec2},
    texture::{draw_texture_ex, Texture2D},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::draw::Drawable;

pub trait Updateable {
    fn update(&mut self, world: &World);
}

pub enum Tile {
    Floor,
    Wall,
}

pub struct CameraControl {
    pub pos: Vec2,
    pub zoom: Vec2,
}
pub struct World {
    pub size: Vec2,
    pub camera: CameraControl,
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

impl World {
    pub fn new(w: f32, h: f32) -> World {
        World {
            size: vec2(w, h),
            camera: CameraControl {
                pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
                zoom: vec2(0.0025, 0.0025),
            },
        }
    }
}
