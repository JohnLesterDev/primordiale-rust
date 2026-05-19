use hecs::World;
use crate::ecs::components::*;
use crate::ecs::resources::GameState;
use super::spawning::{spawn_food, spawn_enemy};

pub fn level_up(world: &mut World, state: &mut GameState) {
    state.current_level += 1;
    state.highest_level = state.highest_level.max(state.current_level);
    state.food_target += 2 + (state.food_target / 3);
    state.enemy_count += 1;
    if state.current_level >= 14 {
        state.enemy_count = 14;
        state.enemy_speed_modifier += 9.0;
    }
    init_level(world, state);
}

pub fn init_level(world: &mut World, state: &mut GameState) {
    let mut to_despawn = Vec::new();
    for (e, _) in world.query_mut::<&Food>() { to_despawn.push(e); }
    for (e, _) in world.query_mut::<&Enemy>() { to_despawn.push(e); }
    for (e, _) in world.query_mut::<&Particle>() { to_despawn.push(e); }
    for e in to_despawn { world.despawn(e).ok(); }

    for _ in 0..state.food_target { spawn_food(world, state, false); }

    let mut player_pos = crate::engine::math::Vec2::zero();
    for (_, (tf, _p)) in world.query_mut::<(&Transform, &Player)>() { player_pos = tf.pos; }
    for _ in 0..state.enemy_count { spawn_enemy(world, state, player_pos); }
}