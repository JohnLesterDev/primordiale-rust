use hecs::World;
use crate::ecs::components::*;
use crate::ecs::resources::GameState;
use crate::engine::math::Vec2;

pub fn update_ai(world: &mut World, _state: &mut GameState, _dt: f32) {
    let mut player_pos = Vec2::zero();
    for (_, (tf, _p)) in world.query_mut::<(&Transform, &Player)>() {
        player_pos = tf.pos + tf.size * 0.5;
    }
    for (_, (tf, vel, enemy)) in world.query_mut::<(&Transform, &mut Velocity, &Enemy)>() {
        let dir = player_pos - tf.pos;
        vel.0 = dir.normalize() * enemy.speed;
    }
}