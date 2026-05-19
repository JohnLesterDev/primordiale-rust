use crate::engine::math::Vec2;
use crate::ecs::globals::EventQueue;
use crate::ecs::particles::queue::ParticleQueue;
use crate::engine::config::GameConfig;

#[derive(Debug, Clone, Copy)]
pub struct DisplayDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct CursorPosition {
    pub pos: Vec2,
}

#[derive(Debug, Clone)]
pub struct GameTimer {
    pub current_tick: u32,
    pub elapsed_time: f32,
    pub timer_text: String,
    pub fps: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Screenshake {
    pub duration: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct LevelProgress {
    pub current_level: u32,
    pub highest_level: u32,
    pub food_target: u32,
    pub enemy_count: u32,
    pub enemy_speed_modifier: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Menu,
    Active,
    GameOver,
}

pub struct Resources {
    pub display: DisplayDimensions,
    pub cursor: CursorPosition,
    pub timer: GameTimer,
    pub shake: Screenshake,
    pub events: EventQueue,
    pub phase: GamePhase,
    pub progress: LevelProgress,
    pub particles: ParticleQueue,
    pub player_entity: Option<hecs::Entity>,
    pub config: GameConfig,
    pub is_level_cleared: bool,
}

impl Resources {
    pub fn new(width: u32, height: u32, initial_tick: u32, config: GameConfig) -> Self {
        Self {
            display: DisplayDimensions { width, height },
            cursor: CursorPosition {
                pos: Vec2::new(width as f32 / 2.0, height as f32 / 2.0),
            },
            timer: GameTimer {
                current_tick: initial_tick,
                elapsed_time: 0.0,
                timer_text: String::new(),
                fps: 0.0,
            },
            shake: Screenshake { duration: 0 },
            events: EventQueue { events: Vec::new() },
            progress: LevelProgress {
                current_level: 1,
                highest_level: 1,
                food_target: 5,
                enemy_count: 1,
                enemy_speed_modifier: 0.0,
            },
            phase: GamePhase::Menu, // Updated default state
            particles: ParticleQueue::new(),
            player_entity: None,
            config,
            is_level_cleared: false,
        }
    }
}
