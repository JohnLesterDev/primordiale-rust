use hecs::CommandBuffer;
use rand::Rng;
use crate::ecs::components::*;
use crate::engine::math::Vec2;
use crate::engine::settings::PARTICLE_SCALE_EXPLOSION;

pub fn spit_particles(
    cmd: &mut CommandBuffer,
    h_scale: f32,
    pos: Vec2,
    color: (u8, u8, u8),
    count: u32,
    max_life_pct: f32,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let mx = rng.gen_range(-20.0..20.0);
        let my = rng.gen_range(-20.0..20.0);
        let vx = mx * 0.5;
        let vy = my * 0.5;

        let s_min = PARTICLE_SCALE_EXPLOSION.0;
        let s_max = PARTICLE_SCALE_EXPLOSION.1;
        let max_lifespan = 2.0 * max_life_pct;

        cmd.spawn((
            Transform { pos: Vec2::new(pos.x, pos.y), size: Vec2::zero() },
            Velocity(Vec2::new(vx, vy)),
            Renderable { color },
            Particle {
                span: rng.gen_range(0.1..max_lifespan),
                size_px: rng.gen_range(s_min..s_max) * h_scale,
                gravity: true,
            },
        ));
    }
}