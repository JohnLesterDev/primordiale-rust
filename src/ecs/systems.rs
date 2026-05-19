use hecs::{World, CommandBuffer};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use rand::Rng;

use crate::ecs::components::*;
use crate::ecs::resources::GameState;
use crate::engine::settings::*;
use crate::engine::math::Vec2;
use crate::engine::audio::AudioSystem;

pub fn spawn_food(world: &mut World, state: &GameState, force_random: bool) {
    let mut rng = rand::thread_rng();
    let fw = FOOD_DIMEN.0 * state.display_height as f32;
    let fh = FOOD_DIMEN.1 * state.display_height as f32;
    let rx = rng.gen_range(0.0..(state.display_width as f32 - fw));
    let ry = rng.gen_range(0.0..(state.display_height as f32 - fh));

    let base_chance = 0.1 + (0.02 * state.current_level as f32);
    let random_move = force_random || rng.gen_range(0.0..1.0) < base_chance;

    let vel = if random_move {
        Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
    } else {
        Vec2::zero()
    };

    let speed = if random_move {
        rng.gen_range(50.0..200.0)
    } else {
        0.0
    };

    world.spawn((
        Transform {
            pos: Vec2::new(rx, ry),
            size: Vec2::new(fw, fh),
        },
        Velocity(vel),
        Renderable { color: (82, 163, 65) },
        Food {
            has_random_movement: random_move,
            speed,
        },
    ));
}

fn spawn_enemy(world: &mut World, state: &GameState, player_pos: Vec2) {
    let mut rng = rand::thread_rng();
    let ew = ENEMY_DIMEN.0 * state.display_height as f32;
    let eh = ENEMY_DIMEN.1 * state.display_height as f32;

    let mut pos = Vec2::zero();
    loop {
        let edge = rng.gen_range(0..4);
        pos.x = match edge {
            0 | 1 => rng.gen_range(0.0..(state.display_width as f32 - ew)),
            2 => 0.0,
            _ => state.display_width as f32 - ew,
        };
        pos.y = match edge {
            0 => 0.0,
            1 => state.display_height as f32 - eh,
            2 | 3 => rng.gen_range(0.0..(state.display_height as f32 - eh)),
            _ => 0.0,
        };
        if pos.distance(&player_pos) > 400.0 {
            break;
        }
    }

    world.spawn((
        Transform {
            pos,
            size: Vec2::new(ew, eh),
        },
        Velocity(Vec2::zero()),
        Renderable { color: (189, 25, 23) },
        Enemy {
            speed: BASE_ENEMY_SPEED + state.enemy_speed_modifier,
        },
    ));
}

fn spit_particles(
    cmd: &mut CommandBuffer,
    state: &GameState,
    pos: Vec2,
    color: (u8, u8, u8),
    count: u32,
    max_life_pct: f32,
) {
    let mut rng = rand::thread_rng();
    let scale = 120.0;

    for _ in 0..count {
        let mx = rng.gen_range(0..=19) as f32;
        let my = rng.gen_range(0..=19) as f32;

        let vx = rng.gen_range((mx * -1.0) * 2.0..mx + 2.0) * scale;
        let vy = rng.gen_range((my * -1.0) * 2.0..my + 2.0) * scale;

        let s_min = PARTICLE_SCALE_EXPLOSION.0 * state.display_height as f32;
        let s_max = PARTICLE_SCALE_EXPLOSION.1 * state.display_height as f32;
        let max_lifespan = 5.0 * max_life_pct;

        cmd.spawn((
            Transform {
                pos: Vec2::new(pos.x, pos.y),
                size: Vec2::zero(),
            },
            Velocity(Vec2::new(vx, vy)),
            Renderable { color },
            Particle {
                span: rng.gen_range(0.1..max_lifespan),
                size_px: rng.gen_range(s_min..s_max),
                gravity: true,
            },
        ));
    }
}

pub fn update_ai(world: &mut World, _state: &mut GameState, _dt: f32) {
    let mut player_pos = Vec2::zero();
    for (_, (tf, _p)) in world.query_mut::<(&Transform, &Player)>() {
        player_pos = tf.pos + tf.size * 0.5;
    }

    for (_, (tf, vel, enemy)) in world.query_mut::<(&Transform, &mut Velocity, &Enemy)>() {
        let dir = player_pos - tf.pos;
        vel.0 = dir.normalize() * enemy.speed;
    }
}

pub fn update_movement(
    world: &mut World,
    state: &GameState,
    dt: f32,
    cmd: &mut CommandBuffer,
) {
    let mut rng = rand::thread_rng();

    // Player movement + trail particles
    for (_, (tf, _p, rend)) in world.query_mut::<(&mut Transform, &Player, &Renderable)>() {
        let target = Vec2::new(
            state.mouse_pos.x - tf.size.x / 2.0,
            state.mouse_pos.y - tf.size.y / 2.0,
        );
        let dir = target - tf.pos;
        tf.pos += dir * PLAYER_LERP_SPEED * dt;

        // Throttle trail particles to avoid excessive heap allocations
        if state.current_tick % 2 == 0 {
            let mx = rng.gen_range(0..=6) as f32;
            let my = rng.gen_range(0..=6) as f32;
            let vx = rng.gen_range((mx * -1.0) * 2.0..mx + 2.0) * 120.0;
            let vy = rng.gen_range((my * -1.0) * 2.0..my + 2.0) * 120.0;

            let t_min = PARTICLE_SCALE_TRAIL.0 * state.display_height as f32;
            let t_max = PARTICLE_SCALE_TRAIL.1 * state.display_height as f32;

            cmd.spawn((
                Transform {
                    pos: tf.pos + tf.size * 0.5,
                    size: Vec2::zero(),
                },
                Velocity(Vec2::new(vx, vy)),
                Renderable { color: rend.color },
                Particle {
                    span: 0.4,
                    size_px: rng.gen_range(t_min..t_max),
                    gravity: false,
                },
            ));
        }
    }

    // Moving food
    for (_, (tf, vel, food)) in world.query_mut::<(&mut Transform, &mut Velocity, &Food)>() {
        if food.has_random_movement {
            tf.pos += vel.0 * food.speed * dt;
            if tf.pos.x < 0.0 {
                vel.0.x *= -1.0;
                tf.pos.x = 0.0;
            }
            if tf.pos.x + tf.size.x > state.display_width as f32 {
                vel.0.x *= -1.0;
                tf.pos.x = state.display_width as f32 - tf.size.x;
            }
            if tf.pos.y < 0.0 {
                vel.0.y *= -1.0;
                tf.pos.y = 0.0;
            }
            if tf.pos.y + tf.size.y > state.display_height as f32 {
                vel.0.y *= -1.0;
                tf.pos.y = state.display_height as f32 - tf.size.y;
            }
        }
    }

    // Enemies (velocity already set by update_ai)
    for (_, (tf, vel, _e)) in world.query_mut::<(&mut Transform, &Velocity, &Enemy)>() {
        tf.pos += vel.0 * dt;
    }

    // Particles
    for (ent, (tf, vel, part)) in
        world.query_mut::<(&mut Transform, &mut Velocity, &mut Particle)>()
    {
        tf.pos += vel.0 * dt;
        if part.gravity {
            vel.0.y += GRAVITY * dt;
        }
        part.span -= dt;
        if part.span <= 0.0 {
            cmd.despawn(ent);
        }
    }
}

pub fn check_collisions(
    world: &mut World,
    state: &mut GameState,
    audio: &AudioSystem,
    cmd: &mut CommandBuffer,
) {
    let mut player_rect = Rect::new(0, 0, 0, 0);
    for (_, (tf, _p)) in world.query_mut::<(&Transform, &Player)>() {
        player_rect = tf.rect();
    }

    // Player vs enemies → game over
    for (_, (tf, _)) in world.query_mut::<(&Transform, &Enemy)>() {
        if tf.rect().has_intersection(player_rect) {
            state.is_game_over = true;
            return;
        }
    }

    // Player eats food
    let total_food = world.query::<&Food>().iter().count();
    let mut eaten_foods = Vec::new();
    for (ent, (tf, _f)) in world.query_mut::<(&Transform, &Food)>() {
        if tf.rect().has_intersection(player_rect) {
            eaten_foods.push((ent, tf.pos));
        }
    }

    let ate_count = eaten_foods.len();
    let is_level_cleared = ate_count > 0 && total_food <= ate_count;

    for (ent, pos) in eaten_foods {
        cmd.despawn(ent);
        state.shake = SHAKE_DURATION;
        audio.play_sfx("eat");
        if !is_level_cleared {
            spit_particles(cmd, state, pos, (82, 163, 65), 220, 1.4);
        }
    }

    if is_level_cleared {
        spit_particles(cmd, state, state.mouse_pos, (255, 255, 255), 1000, 1.0);
        level_up(world, state, audio);
    }

    // No enemy‑enemy collision – MVP simplicity
}

fn level_up(world: &mut World, state: &mut GameState, audio: &AudioSystem) {
    state.current_level += 1;
    state.highest_level = state.highest_level.max(state.current_level);
    state.food_target += 2 + (state.food_target / 3);
    state.enemy_count += 1;
    if state.current_level >= 14 {
        state.enemy_count = 14;
        state.enemy_speed_modifier += 9.0;
    }
    audio.play_sfx("level");
    init_level(world, state);
}

pub fn init_level(world: &mut World, state: &mut GameState) {
    let mut to_despawn = Vec::new();
    for (e, _) in world.query_mut::<&Food>() {
        to_despawn.push(e);
    }
    for (e, _) in world.query_mut::<&Enemy>() {
        to_despawn.push(e);
    }
    for e in to_despawn {
        world.despawn(e).ok();
    }

    for _ in 0..state.food_target {
        spawn_food(world, state, false);
    }

    let mut player_pos = Vec2::zero();
    for (_, (tf, _p)) in world.query_mut::<(&Transform, &Player)>() {
        player_pos = tf.pos;
    }

    for _ in 0..state.enemy_count {
        spawn_enemy(world, state, player_pos);
    }
}

pub fn render(world: &mut World, state: &mut GameState, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut rng = rand::thread_rng();
    if state.shake > 0 {
        state.shake -= 1;
        state.shake_offset = Vec2::new(
            rng.gen_range(-SHAKE_INTENSITY..SHAKE_INTENSITY) as f32,
            rng.gen_range(-SHAKE_INTENSITY..SHAKE_INTENSITY) as f32,
        );
    } else {
        state.shake_offset = Vec2::zero();
    }

    let ox = state.shake_offset.x as i32;
    let oy = state.shake_offset.y as i32;

    // Food
    for (_, (tf, rend, _)) in world.query_mut::<(&Transform, &Renderable, &Food)>() {
        canvas.set_draw_color(Color::RGB(rend.color.0, rend.color.1, rend.color.2));
        canvas
            .fill_rect(Rect::new(
                tf.pos.x as i32 + ox,
                tf.pos.y as i32 + oy,
                tf.size.x as u32,
                tf.size.y as u32,
            ))
            .ok();
    }

    // Enemies – just red rectangles, no boss projectiles
    for (_, (tf, rend, _)) in world.query_mut::<(&Transform, &Renderable, &Enemy)>() {
        canvas.set_draw_color(Color::RGB(rend.color.0, rend.color.1, rend.color.2));
        canvas
            .fill_rect(Rect::new(
                tf.pos.x as i32 + ox,
                tf.pos.y as i32 + oy,
                tf.size.x as u32,
                tf.size.y as u32,
            ))
            .ok();
    }

    // Player
    for (_, (tf, rend, _)) in world.query_mut::<(&Transform, &Renderable, &Player)>() {
        canvas.set_draw_color(Color::RGB(rend.color.0, rend.color.1, rend.color.2));
        canvas
            .fill_rect(Rect::new(
                tf.pos.x as i32 + ox,
                tf.pos.y as i32 + oy,
                tf.size.x as u32,
                tf.size.y as u32,
            ))
            .ok();
    }

    // Particles as filled circles
    for (_, (tf, rend, part)) in world.query_mut::<(&Transform, &Renderable, &Particle)>() {
        let rad = part.size_px as i16;
        let cx = (tf.pos.x as i32 + ox) as i16;
        let cy = (tf.pos.y as i32 + oy) as i16;
        canvas
            .filled_circle(cx, cy, rad, Color::RGB(rend.color.0, rend.color.1, rend.color.2))
            .ok();
    }
}