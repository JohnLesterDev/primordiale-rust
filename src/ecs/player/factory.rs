use hecs::{World, Entity};
use crate::ecs::player::Player;
use crate::ecs::shared::{Transform, Renderable};
use crate::engine::math::Vec2;
use crate::engine::config::GameConfig;

pub fn spawn_player(world: &mut World, config: &GameConfig) -> Entity {
    let pw = config.entities.player_dimen.0;
    let ph = config.entities.player_dimen.1;
    
    world.spawn((
        Transform { pos: Vec2::zero(), size: Vec2::new(pw, ph) },
        Renderable { color: (106, 109, 115) },
        Player
    ))
}