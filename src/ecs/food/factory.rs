use hecs::World;
use rand::Rng;
use crate::ecs::food::component::Food;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::velocity::Velocity;
use crate::ecs::shared::renderable::Renderable;
use crate::ecs::resources::GameState;
use crate::engine::settings::FOOD_DIMEN;
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