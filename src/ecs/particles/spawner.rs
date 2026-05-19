use hecs::CommandBuffer;
use rand::Rng;
use crate::ecs::particles::Particle;
use crate::ecs::particles::queue::ParticleQueue;
use crate::ecs::shared::{Transform, Velocity, Renderable};
use crate::engine::math::Vec2;
use crate::engine::config::GameConfig;

pub fn drain_particle_queue(config: &GameConfig, queue: &mut ParticleQueue, cmd: &mut CommandBuffer) {
    let mut rng = rand::thread_rng();
    
    for req in queue.requests.drain(..) {
        for _ in 0..req.count {
            let mx = rng.gen_range(-20.0..20.0);
            let my = rng.gen_range(-20.0..20.0);
            let vx = mx * 0.5;
            let vy = my * 0.5;

            let s_min = config.juice.particle_scale_explosion_min;
            let s_max = config.juice.particle_scale_explosion_max;
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