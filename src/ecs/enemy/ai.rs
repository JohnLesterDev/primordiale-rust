use hecs::World;
use crate::ecs::enemy::component::Enemy;
use crate::ecs::player::component::Player;
use crate::ecs::shared::transform::Transform;
use crate::ecs::shared::velocity::Velocity;

pub fn update_ai(world: &mut World, _dt: f32) {
    let mut player_pos = crate::engine::math::Vec2::zero();
    
    // Dynamically query target coordinates via the domain component
    for (_, (tf, _p)) in world.query_mut::<(&Transform, &Player)>() {
        player_pos = tf.pos + tf.size * 0.5;
    }
    
    for (_, (tf, vel, enemy)) in world.query_mut::<(&Transform, &mut Velocity, &Enemy)>() {
        let dir = player_pos - tf.pos;
        vel.0 = dir.normalize() * enemy.speed;
    }
}