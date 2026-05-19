// src/ecs/systems/render.rs

use hecs::World;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use rand::Rng;

use crate::ecs::components::{Transform, Renderable, Particle};
use crate::ecs::resources::GameState;
use crate::engine::settings::SHAKE_INTENSITY;

pub fn draw_world(world: &mut World, state: &GameState, canvas: &mut WindowCanvas) {
    let h_scale = state.display_height as f32;

    let (ox, oy) = if state.shake > 0 && !state.is_game_over {
        let mut rng = rand::thread_rng();
        (
            (rng.gen_range(-SHAKE_INTENSITY..SHAKE_INTENSITY) * h_scale) as i32,
            (rng.gen_range(-SHAKE_INTENSITY..SHAKE_INTENSITY) * h_scale) as i32,
        )
    } else {
        (0, 0)
    };

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if state.is_game_over {
        return;
    }

    for (_, (tf, rend)) in world.query_mut::<(&Transform, &Renderable)>() {
        canvas.set_draw_color(Color::RGB(rend.color.0, rend.color.1, rend.color.2));
        canvas.fill_rect(Rect::new(
            (tf.pos.x * h_scale) as i32 + ox,
            (tf.pos.y * h_scale) as i32 + oy,
            (tf.size.x * h_scale) as u32,
            (tf.size.y * h_scale) as u32,
        )).ok();
    }

    for (_, (tf, rend, part)) in world.query_mut::<(&Transform, &Renderable, &Particle)>() {
        canvas.set_draw_color(Color::RGB(rend.color.0, rend.color.1, rend.color.2));
        canvas.fill_rect(Rect::new(
            (tf.pos.x * h_scale) as i32 + ox,
            (tf.pos.y * h_scale) as i32 + oy,
            part.size_px.max(2.0) as u32,
            part.size_px.max(2.0) as u32,
        )).ok();
    }
}