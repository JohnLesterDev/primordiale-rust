use hecs::World;
use crate::ecs::food::component::Food;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::velocity::Velocity;
use crate::resources::Resources;

pub fn update_food_movement(world: &mut World, res: &Resources, dt: f32) {
    let aspect_ratio = res.display.width as f32 / res.display.height as f32;

    for (_, (tf, vel, food)) in world.query_mut::<(&mut Transform, &mut Velocity, &Food)>() {
        if food.has_random_movement {
            tf.pos += vel.0 * food.speed * dt;
            if tf.pos.x < 0.0 { vel.0.x *= -1.0; tf.pos.x = 0.0; }
            if tf.pos.x + tf.size.x > aspect_ratio { vel.0.x *= -1.0; tf.pos.x = aspect_ratio - tf.size.x; }
            if tf.pos.y < 0.0 { vel.0.y *= -1.0; tf.pos.y = 0.0; }
            if tf.pos.y + tf.size.y > 1.0 { vel.0.y *= -1.0; tf.pos.y = 1.0 - tf.size.y; }
        }
    }
}