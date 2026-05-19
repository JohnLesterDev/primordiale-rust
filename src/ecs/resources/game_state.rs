use crate::engine::math::Vec2;
use super::GameEvent;

pub struct GameState {
    pub display_width: u32,
    pub display_height: u32,
    pub current_level: u32,
    pub highest_level: u32,
    pub food_target: u32,
    pub enemy_count: u32,
    pub is_game_over: bool,
    pub elapsed_time: f32,
    pub start_time: u32,
    pub current_tick: u32,
    pub shake: u32,
    pub shake_offset: Vec2,
    pub mouse_pos: Vec2,
    pub timer_text: String,
    pub enemy_speed_modifier: f32,
    pub fps: f32,
    pub events: Vec<GameEvent>,
}