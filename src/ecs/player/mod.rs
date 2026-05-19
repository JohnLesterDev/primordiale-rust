pub mod component;
pub mod movement;
pub mod factory;

pub use component::Player;
pub use movement::update_player_movement;
pub use factory::spawn_player;