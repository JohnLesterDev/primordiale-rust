use hecs::CommandBuffer;
use rand::Rng;
use crate::ecs::particles::component::Particle;
use crate::ecs::particles::queue::ParticleQueue;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::velocity::Velocity;
use crate::ecs::shared::renderable::Renderable;
use crate::engine::math::Vec2;
use crate::engine::settings::PARTICLE_SCALE_EXPLOSION;

pub fn drain_particle_queue(queue: &mut ParticleQueue, cmd: &mut CommandBuffer) {
    let mut rng = rand::thread_rng();
    
    for req in queue.requests.drain(..) {
        for _ in 0..req.count {
            let mx = rng.gen_range(-20.0..20.0);
            let my = rng.gen_range(-20.0..20.0);
            let vx = mx * 0.5;
            let vy = my * 0.5;

            let s_min = PARTICLE_SCALE_EXPLOSION.0;
            let s_max = PARTICLE_SCALE_EXPLOSION.1;
            let max_lifespan = 2.0 * req.max_life_pct;

            cmd.spawn((
                Transform { pos: Vec2::new(req.pos.x, req.pos.y), size: Vec2::zero() },
                Velocity(Vec2::new(vx, vy)),
                Renderable { color: req.color },
                Particle {
                    span: rng.gen_range(0.1..max_lifespan),
                    size_px: rng.gen_range(s_min..s_max) * req.h_scale,
                    gravity: true,
                },
            ));
        }
    }
}