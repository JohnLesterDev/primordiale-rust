use hecs::World;
use rand::Rng;
use crate::ecs::enemy::Enemy;
use crate::ecs::shared::{Transform, Velocity, Renderable};
use crate::resources::Resources;
use crate::engine::math::Vec2;

pub fn spawn_enemy(world: &mut World, res: &Resources, player_pos: Vec2) {
    let mut rng = rand::thread_rng();
    let aspect_ratio = res.display.width as f32 / res.display.height as f32;
    let ew = res.config.entities.enemy_dimen.0;
    let eh = res.config.entities.enemy_dimen.1;

    let mut pos = Vec2::zero();
    loop {
        let edge = rng.gen_range(0..4);
        pos.x = match edge {
            0 | 1 => rng.gen_range(0.0..(aspect_ratio - ew)),
            2 => 0.0,
            _ => aspect_ratio - ew,
        };
        pos.y = match edge {
            0 => 0.0,
            1 => 1.0 - eh,
            _ => rng.gen_range(0.0..(1.0 - eh)),
        };
        if pos.distance(&player_pos) > 0.4 { break; }
    }

    world.spawn((
        Transform { pos, size: Vec2::new(ew, eh) },
        Velocity(Vec2::zero()),
        Renderable { color: (189, 25, 23) },
        Enemy { speed: res.config.entities.base_enemy_speed + (res.progress.enemy_speed_modifier / res.display.height as f32) },
    ));
}