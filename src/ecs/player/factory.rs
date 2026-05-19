use hecs::World;
use crate::ecs::player::component::Player;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::renderable::Renderable;
use crate::engine::math::Vec2;
use crate::engine::settings::PLAYER_DIMEN;

pub fn spawn_player(world: &mut World) {
    let pw = PLAYER_DIMEN.0;
    let ph = PLAYER_DIMEN.1;
    
    world.spawn((
        Transform { pos: Vec2::zero(), size: Vec2::new(pw, ph) },
        Renderable { color: (106, 109, 115) },
        Player
    ));
}