// src/ecs/globals/dispatch.rs

use hecs::{World, CommandBuffer};
use crate::ecs::globals::event_queue::GameEvent;
use crate::ecs::systems::particles::spit_particles;
use crate::ecs::systems::level_up;          // Already imported
use crate::ecs::resources::GameState;         // Added reference type
use crate::engine::audio::AudioSystem;
use crate::engine::math::Vec2;
use crate::resources::{Resources, GamePhase};

pub fn dispatch_events(
    world: &mut World,
    res: &mut Resources,
    state: &mut GameState,                   // Added tracking state
    audio: &AudioSystem,
    cmd: &mut CommandBuffer,
) {
    let h_scale = res.display.height as f32;

    for event in res.events.events.drain(..) {
        match event {
            GameEvent::Eat(pos) => {
                audio.play_sfx("eat");
                spit_particles(cmd, h_scale, pos, (82, 163, 65), 100, 1.0);
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
                spit_particles(cmd, h_scale, m_pos, (255, 255, 255), 300, 1.2);
                
                // Task 6 Fix: Execute the actual progression system mutation
                level_up(world, state);
            }
        }
    }
}