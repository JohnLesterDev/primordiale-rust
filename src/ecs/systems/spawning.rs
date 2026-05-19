use hecs::World;
use rand::Rng;
use crate::ecs::components::*;
use crate::ecs::resources::GameState;
use crate::engine::settings::*;
use crate::engine::math::Vec2;

pub fn spawn_food(world: &mut World, state: &GameState, force_random: bool) {
    let mut rng = rand::thread_rng();
    let aspect_ratio = state.display_width as f32 / state.display_height as f32;

    let fw = FOOD_DIMEN.0;
    let fh = FOOD_DIMEN.1;
    let rx = rng.gen_range(0.0..(aspect_ratio - fw));
    let ry = rng.gen_range(0.0..(1.0 - fh));

    let base_chance = 0.1 + (0.02 * state.current_level as f32);
    let random_move = force_random || rng.gen_range(0.0..1.0) < base_chance;

    let vel = if random_move {
        Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
    } else {
        Vec2::zero()
    };

    let speed = if random_move { rng.gen_range(0.05..0.2) } else { 0.0 };

    world.spawn((
        Transform { pos: Vec2::new(rx, ry), size: Vec2::new(fw, fh) },
        Velocity(vel),
        Renderable { color: (82, 163, 65) },
        Food { has_random_movement: random_move, speed },
    ));
}

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