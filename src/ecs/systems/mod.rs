mod movement;
mod collision;

// Local cross-cutting orchestrators
pub use movement::update_movement;
pub use collision::check_collisions;

// Domain forward-routing hooks
pub use crate::ecs::player::movement::update_player_movement;
pub use crate::ecs::enemy::ai::update_ai;
pub use crate::ecs::enemy::factory::spawn_enemy;
pub use crate::ecs::food::movement::update_food_movement;
pub use crate::ecs::food::factory::spawn_food;
pub use crate::ecs::particles::update::update_particles;
pub use crate::ecs::particles::spawner::drain_particle_queue;
pub use crate::ecs::level::progression::{level_up, init_level};