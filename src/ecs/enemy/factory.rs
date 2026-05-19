use hecs::World;
use rand::Rng;
use crate::ecs::enemy::component::Enemy;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::velocity::Velocity;
use crate::ecs::shared::renderable::Renderable;
use crate::ecs::resources::GameState;
use crate::engine::settings::{ENEMY_DIMEN, BASE_ENEMY_SPEED};
use crate::engine::math::Vec2;

pub fn spawn_enemy(world: &mut World, state: &GameState, player_pos: Vec2) {
    let mut rng = rand::thread_rng();
    let aspect_ratio = state.display_width as f32 / state.display_height as f32;
    let ew = ENEMY_DIMEN.0;
    let eh = ENEMY_DIMEN.1;

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
        Enemy { speed: BASE_ENEMY_SPEED + (state.enemy_speed_modifier / state.display_height as f32) },
    ));
}