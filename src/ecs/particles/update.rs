use hecs::{World, CommandBuffer};
use crate::ecs::particles::Particle;
use crate::ecs::shared::{Transform, Velocity};
use crate::resources::Resources;

pub fn update_particles(world: &mut World, res: &Resources, dt: f32, cmd: &mut CommandBuffer) {
    for (ent, (tf, vel, part)) in world.query_mut::<(&mut Transform, &mut Velocity, &mut Particle)>() {
        tf.pos += vel.0 * dt;
        if part.gravity { 
            vel.0.y += res.config.physics.gravity * dt; 
        }
        part.span -= dt;
        if part.span <= 0.0 { 
            cmd.despawn(ent); 
        }
    }
}