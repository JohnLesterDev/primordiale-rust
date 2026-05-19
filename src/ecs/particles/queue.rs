use crate::engine::math::Vec2;

#[derive(Debug, Clone)]
pub struct ParticleSpawnRequest {
    pub h_scale: f32,
    pub pos: Vec2,
    pub color: (u8, u8, u8),
    pub count: u32,
    pub max_life_pct: f32,
}

pub struct ParticleQueue {
    pub requests: Vec<ParticleSpawnRequest>,
}

impl ParticleQueue {
    pub fn new() -> Self {
        Self { requests: Vec::new() }
    }
    
    pub fn push(&mut self, req: ParticleSpawnRequest) {
        self.requests.push(req);
    }
}