use hecs::World;
use rand::Rng;
use crate::ecs::food::Food;
use crate::ecs::shared::{Transform, Velocity, Renderable};
use crate::resources::Resources;
use crate::engine::math::Vec2;

pub fn spawn_food(world: &mut World, res: &Resources, force_random: bool) {
    let mut rng = rand::thread_rng();
    let aspect_ratio = res.display.width as f32 / res.display.height as f32;

    let fw = res.config.entities.food_dimen.0;
    let fh = res.config.entities.food_dimen.1;
    let rx = rng.gen_range(0.0..(aspect_ratio - fw));
    let ry = rng.gen_range(0.0..(1.0 - fh));

    let base_chance = 0.1 + (0.02 * res.progress.current_level as f32);
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