use hecs::{World, CommandBuffer};
use crate::ecs::components::*;
use crate::ecs::resources::{GameState, GameEvent};
use crate::engine::math::Vec2;
use crate::engine::settings::SHAKE_DURATION;

use super::level::level_up;  // we import level_up from sibling module

pub fn check_collisions(world: &mut World, state: &mut GameState, cmd: &mut CommandBuffer) {
    let mut player_pos = Vec2::zero();
    let mut player_size = Vec2::zero();

    for (_, (tf, _p)) in world.query_mut::<(&Transform, &Player)>() {
        player_pos = tf.pos;
        player_size = tf.size;
    }

    let intersects = |p1: Vec2, s1: Vec2, p2: Vec2, s2: Vec2| -> bool {
        p1.x < p2.x + s2.x && p1.x + s1.x > p2.x && p1.y < p2.y + s2.y && p1.y + s1.y > p2.y
    };

    // Enemy collisions
    for (_, (tf, _)) in world.query_mut::<(&Transform, &Enemy)>() {
        if intersects(player_pos, player_size, tf.pos, tf.size) {
            state.is_game_over = true;
            state.events.push(GameEvent::Kill);
            return;
        }
    }

    // Food collisions
    let mut eaten_foods = Vec::new();
    for (ent, (tf, _f)) in world.query_mut::<(&Transform, &Food)>() {
        if intersects(player_pos, player_size, tf.pos, tf.size) {
            eaten_foods.push((ent, tf.pos));
        }
    }

    let total_food = world.query::<&Food>().iter().count();
    let ate_count = eaten_foods.len();
    let is_level_cleared = ate_count > 0 && total_food <= ate_count;

    for (ent, pos) in eaten_foods {
        cmd.despawn(ent);
        state.shake = SHAKE_DURATION;   
        state.events.push(GameEvent::Eat(pos));
    }

    if is_level_cleared {
        state.events.push(GameEvent::LevelUp);
        level_up(world, state);
    }
}