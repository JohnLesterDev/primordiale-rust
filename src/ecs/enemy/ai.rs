use hecs::World;
use crate::ecs::enemy::Enemy;
use crate::ecs::shared::{Transform, Velocity};
use crate::resources::Resources;

pub fn update_ai(world: &mut World, res: &Resources) {
    let mut player_pos = crate::engine::math::Vec2::zero();
    
    if let Some(player) = res.player_entity {
        if let Ok(tf) = world.get::<&Transform>(player) {
            player_pos = tf.pos + tf.size * 0.5;
        }
    }

    for (_, (tf, vel, enemy)) in world.query_mut::<(&Transform, &mut Velocity, &Enemy)>() {
        let dir = player_pos - tf.pos;
        vel.0 = dir.normalize() * enemy.speed;
    }
}