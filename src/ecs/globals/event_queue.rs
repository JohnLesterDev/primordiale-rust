use crate::engine::math::Vec2;

#[derive(Debug, Clone)]
pub enum GameEvent {
    Eat(Vec2),
    Kill,
    LevelUp,
}

#[derive(Debug, Clone)]
pub struct EventQueue {
    pub events: Vec<GameEvent>,
}