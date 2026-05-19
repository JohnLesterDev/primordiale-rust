use hecs::World;
use crate::ecs::shared::Transform;
use crate::resources::Resources;
use crate::engine::math::Vec2;

pub fn update_player_movement(world: &mut World, res: &Resources, dt: f32) {
    if let Some(player) = res.player_entity {
        if let Ok(mut tf) = world.get::<&mut Transform>(player) {
            let target = Vec2::new(
                (res.cursor.pos.x / res.display.height as f32) - tf.size.x / 2.0,
                (res.cursor.pos.y / res.display.height as f32) - tf.size.y / 2.0,
            );
            let dir = target - tf.pos;
            tf.pos += dir * res.config.entities.player_lerp_speed * dt;
        }
    }
}