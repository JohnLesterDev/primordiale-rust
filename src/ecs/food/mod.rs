pub mod component;
pub mod movement;
pub mod factory;

pub use component::Food;
pub use movement::update_food_movement;
pub use factory::spawn_food;