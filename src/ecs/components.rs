use crate::engine::math::Vec2;
use sdl2::rect::Rect;

pub struct Transform {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Transform {
    pub fn rect(&self) -> Rect {
        Rect::new(
            self.pos.x as i32,
            self.pos.y as i32,
            self.size.x as u32, 
            self.size.y as u32 
        )
    }
}

pub struct Velocity(pub Vec2);
pub struct Renderable { pub color: (u8, u8, u8) }

pub struct Player;

pub struct Food {
    pub has_random_movement: bool,
    pub speed: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Enemy {
    pub speed: f32,
}

pub struct Particle {
    pub span: f32,
    pub size_px: f32,
    pub gravity: bool,
}