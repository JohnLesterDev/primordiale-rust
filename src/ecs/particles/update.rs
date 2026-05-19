use hecs::{World, CommandBuffer};
use crate::ecs::particles::component::Particle;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::velocity::Velocity;
use crate::engine::settings::GRAVITY;

pub fn update_particles(world: &mut World, dt: f32, cmd: &mut CommandBuffer) {
    for (ent, (tf, vel, part)) in world.query_mut::<(&mut Transform, &mut Velocity, &mut Particle)>() {
        tf.pos += vel.0 * dt;
        if part.gravity { 
            vel.0.y += GRAVITY * dt; 
        }
        part.span -= dt;
        if part.span <= 0.0 { 
            cmd.despawn(ent); 
        }
    }
}