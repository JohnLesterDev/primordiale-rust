mod ai;
mod movement;
mod collision;
mod spawning;
mod level;
pub mod particles;
mod render; 

pub use ai::update_ai;
pub use movement::update_movement;
pub use collision::check_collisions;
pub use spawning::{spawn_food, spawn_enemy};
pub use level::{level_up, init_level};
pub use render::draw_world;
