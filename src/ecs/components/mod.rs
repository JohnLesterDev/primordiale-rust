pub use crate::ecs::shared::transform::Transform;
pub use crate::ecs::shared::velocity::Velocity;
pub use crate::ecs::shared::renderable::Renderable;
pub use crate::ecs::player::component::Player;
pub use crate::ecs::enemy::component::Enemy;
pub use crate::ecs::food::component::Food;

// Backward-compatible re-export link
pub use crate::ecs::particles::component::Particle;