use hecs::World;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::velocity::Velocity;
use crate::ecs::enemy::component::Enemy;

pub fn update_movement(world: &mut World, dt: f32) {
    // Basic linear translation fallback pass for moving entities
    for (_, (tf, vel, _e)) in world.query_mut::<(&mut Transform, &Velocity, &Enemy)>() {
        tf.pos += vel.0 * dt;
    }
}