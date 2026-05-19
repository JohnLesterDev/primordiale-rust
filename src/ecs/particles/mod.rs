pub mod component;
pub mod queue;
pub mod spawner;
pub mod update;

pub use component::Particle;
pub use queue::{ParticleSpawnRequest, ParticleQueue};
pub use spawner::drain_particle_queue;
pub use update::update_particles;