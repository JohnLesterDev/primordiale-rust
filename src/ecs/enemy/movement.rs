use hecs::World;
use crate::ecs::shared::Transform;
use crate::ecs::shared::Velocity;
use crate::ecs::enemy::Enemy;

pub fn update_enemy_movement(world: &mut World, dt: f32) {
    for (_, (tf, vel, _e)) in world.query_mut::<(&mut Transform, &Velocity, &Enemy)>() {
        tf.pos += vel.0 * dt;
    }
}