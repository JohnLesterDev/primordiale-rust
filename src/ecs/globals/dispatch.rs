use hecs::World;
use crate::ecs::globals::event_queue::GameEvent;
use crate::engine::audio::AudioSystem;
use crate::engine::math::Vec2;
use crate::resources::{Resources, GamePhase};
use crate::ecs::particles::queue::ParticleSpawnRequest;

pub fn dispatch_events(
    _world: &mut World, 
    res: &mut Resources,
    audio: &AudioSystem,
) {
    let h_scale = res.display.height as f32;

    for event in res.events.events.drain(..) {
        match event {
            GameEvent::Eat(pos) => {
                audio.play_sfx("eat");
                res.particles.push(ParticleSpawnRequest {
                    h_scale,
                    pos,
                    color: (82, 163, 65),
                    count: 100,
                    max_life_pct: 1.0,
                });
            }
            GameEvent::Kill => {
                audio.play_sfx("kill");
                res.phase = GamePhase::GameOver;
            }
            GameEvent::LevelUp => {
                audio.play_sfx("level");
                
                let m_pos = Vec2::new(
                    res.cursor.pos.x / h_scale,
                    res.cursor.pos.y / h_scale,
                );
                res.particles.push(ParticleSpawnRequest {
                    h_scale,
                    pos: m_pos,
                    color: (255, 255, 255),
                    count: 300,
                    max_life_pct: 1.2,
                });
                
                res.is_level_cleared = true;
            }
        }
    }
}