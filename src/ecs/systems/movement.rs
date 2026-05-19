use hecs::{World, CommandBuffer};
use crate::ecs::components::*;
use crate::ecs::resources::GameState;
use crate::engine::settings::*;
use crate::engine::math::Vec2;

pub fn update_movement(world: &mut World, state: &GameState, dt: f32, cmd: &mut CommandBuffer) {
    let aspect_ratio = state.display_width as f32 / state.display_height as f32;

    // Player
    for (_, (tf, _p, _rend)) in world.query_mut::<(&mut Transform, &Player, &Renderable)>() {
        let target = Vec2::new(
            (state.mouse_pos.x / state.display_height as f32) - tf.size.x / 2.0,
            (state.mouse_pos.y / state.display_height as f32) - tf.size.y / 2.0,
        );
        let dir = target - tf.pos;
        tf.pos += dir * PLAYER_LERP_SPEED * dt;
    }

    // Food
    for (_, (tf, vel, food)) in world.query_mut::<(&mut Transform, &mut Velocity, &Food)>() {
        if food.has_random_movement {
            tf.pos += vel.0 * food.speed * dt;
            if tf.pos.x < 0.0 { vel.0.x *= -1.0; tf.pos.x = 0.0; }
            if tf.pos.x + tf.size.x > aspect_ratio { vel.0.x *= -1.0; tf.pos.x = aspect_ratio - tf.size.x; }
            if tf.pos.y < 0.0 { vel.0.y *= -1.0; tf.pos.y = 0.0; }
            if tf.pos.y + tf.size.y > 1.0 { vel.0.y *= -1.0; tf.pos.y = 1.0 - tf.size.y; }
        }
    }

    // Enemies
    for (_, (tf, vel, _e)) in world.query_mut::<(&mut Transform, &Velocity, &Enemy)>() {
        tf.pos += vel.0 * dt;
    }

    // Particles
    for (ent, (tf, vel, part)) in world.query_mut::<(&mut Transform, &mut Velocity, &mut Particle)>() {
        tf.pos += vel.0 * dt;
        if part.gravity { vel.0.y += GRAVITY * dt; }
        part.span -= dt;
        if part.span <= 0.0 { cmd.despawn(ent); }
    }
}