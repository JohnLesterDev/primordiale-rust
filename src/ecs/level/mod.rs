pub mod progression;

pub use progression::{init_level, level_up};

#[derive(Debug, Clone, Copy)]
pub struct LevelProgress {
    pub current_level: u32,
    pub highest_level: u32,
    pub food_target: u32,
    pub enemy_count: u32,
    pub enemy_speed_modifier: f32,
}