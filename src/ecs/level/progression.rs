use hecs::World;
use crate::resources::Resources;
use crate::ecs::food::factory::spawn_food;
use crate::ecs::enemy::factory::spawn_enemy;
use crate::ecs::food::Food;
use crate::ecs::enemy::Enemy;
use crate::ecs::particles::Particle;

pub fn level_up(world: &mut World, res: &mut Resources) {
    res.progress.current_level += 1;
    res.progress.highest_level = res.progress.highest_level.max(res.progress.current_level);
    res.progress.food_target += 2 + (res.progress.food_target / 3);
    res.progress.enemy_count += 1;
    if res.progress.current_level >= 14 {
        res.progress.enemy_count = 14;
        res.progress.enemy_speed_modifier += 9.0;
    }
    init_level(world, res);
}

pub fn init_level(world: &mut World, res: &mut Resources) {
    let mut to_despawn = Vec::new();
    for (e, _) in world.query_mut::<&Food>() { to_despawn.push(e); }
    for (e, _) in world.query_mut::<&Enemy>() { to_despawn.push(e); }
    for (e, _) in world.query_mut::<&Particle>() { to_despawn.push(e); }
    for e in to_despawn { world.despawn(e).ok(); }

    for _ in 0..res.progress.food_target { 
        spawn_food(world, res, false); 
    }

    let mut player_pos = crate::engine::math::Vec2::zero();
    for (_, (tf, _p)) in world.query_mut::<(&crate::ecs::shared::Transform, &crate::ecs::player::Player)>() { 
        player_pos = tf.pos; 
    }
    for _ in 0..res.progress.enemy_count { 
        spawn_enemy(world, res, player_pos); 
    }
}