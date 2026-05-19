use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub target_fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub gravity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityConfig {
    pub player_dimen: (f32, f32),
    pub food_dimen: (f32, f32),
    pub enemy_dimen: (f32, f32),
    pub base_enemy_speed: f32,
    pub player_lerp_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JuiceConfig {
    pub shake_intensity: f32,
    pub shake_duration: u32,
    pub particle_scale_explosion_min: f32,
    pub particle_scale_explosion_max: f32,
}

/// Consolidated engine configurations structure mapping tunable constants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub window: WindowConfig,
    pub physics: PhysicsConfig,
    pub entities: EntityConfig,
    pub juice: JuiceConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                target_fps: 120, // matches settings::FPS
            },
            physics: PhysicsConfig {
                gravity: 1.2, // matches settings::GRAVITY
            },
            entities: EntityConfig {
                player_dimen: (0.04, 0.04),      // matches settings::PLAYER_DIMEN
                food_dimen: (0.017, 0.017),      // matches settings::FOOD_DIMEN
                enemy_dimen: (0.025, 0.025),     // matches settings::ENEMY_DIMEN
                base_enemy_speed: 0.21,          // matches settings::BASE_ENEMY_SPEED
                player_lerp_speed: 16.0,         // matches settings::PLAYER_LERP_SPEED
            },
            juice: JuiceConfig {
                shake_intensity: 0.008,          // matches settings::SHAKE_INTENSITY
                shake_duration: 50,              // matches settings::SHAKE_DURATION
                particle_scale_explosion_min: 0.002, // matches settings::PARTICLE_SCALE_EXPLOSION.0
                particle_scale_explosion_max: 0.005, // matches settings::PARTICLE_SCALE_EXPLOSION.1
            },
        }
    }
}