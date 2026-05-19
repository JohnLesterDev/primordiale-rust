use hecs::World;
use crate::ecs::player::Player;
use crate::ecs::shared::Transform;
use crate::resources::Resources;
use crate::engine::settings::PLAYER_LERP_SPEED;
use crate::engine::math::Vec2;

pub fn update_player_movement(world: &mut World, res: &Resources, dt: f32) {
    for (_, (tf, _p)) in world.query_mut::<(&mut Transform, &Player)>() {
        let target = Vec2::new(
            (res.cursor.pos.x / res.display.height as f32) - tf.size.x / 2.0,
            (res.cursor.pos.y / res.display.height as f32) - tf.size.y / 2.0,
        );
        let dir = target - tf.pos;
        tf.pos += dir * PLAYER_LERP_SPEED * dt;
    }
}