use hecs::World;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use rand::Rng;

use crate::ecs::components::{Transform, Renderable, Particle};
use crate::engine::settings::SHAKE_INTENSITY;
use crate::resources::{DisplayDimensions, Screenshake, GamePhase};

pub fn draw_world(
    world: &World,
    display: &DisplayDimensions,
    shake: &Screenshake,
    phase: GamePhase,
    canvas: &mut WindowCanvas,
) {
    let h_scale = display.height as f32;

    // Screenshake offset calculation
    let (ox, oy) = if shake.duration > 0 && phase != GamePhase::GameOver {
        let mut rng = rand::thread_rng();
        (
            (rng.gen_range(-SHAKE_INTENSITY..SHAKE_INTENSITY) * h_scale) as i32,
            (rng.gen_range(-SHAKE_INTENSITY..SHAKE_INTENSITY) * h_scale) as i32,
        )
    } else {
        (0, 0)
    };

    // Clean viewport background pass
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if phase == GamePhase::GameOver {
        return;
    }

    // Immutable queries to prevent world corruption during draw pass
    for (_, (tf, rend)) in world.query::<(&Transform, &Renderable)>().iter() {
        canvas.set_draw_color(Color::RGB(rend.color.0, rend.color.1, rend.color.2));
        canvas.fill_rect(Rect::new(
            (tf.pos.x * h_scale) as i32 + ox,
            (tf.pos.y * h_scale) as i32 + oy,
            (tf.size.x * h_scale) as u32,
            (tf.size.y * h_scale) as u32,
        )).ok();
    }

    for (_, (tf, rend, part)) in world.query::<(&Transform, &Renderable, &Particle)>().iter() {
        canvas.set_draw_color(Color::RGB(rend.color.0, rend.color.1, rend.color.2));
        canvas.fill_rect(Rect::new(
            (tf.pos.x * h_scale) as i32 + ox,
            (tf.pos.y * h_scale) as i32 + oy,
            part.size_px.max(2.0) as u32,
            part.size_px.max(2.0) as u32,
        )).ok();
    }
}