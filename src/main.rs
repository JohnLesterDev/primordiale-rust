mod resources;
mod engine;
mod ecs;
mod render;

use std::fs;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use hecs::{World, CommandBuffer};

use engine::math::Vec2;
use engine::audio::AudioSystem;
use engine::config::GameConfig;
use ecs::level::{init_level, level_up};
use ecs::player::{spawn_player, update_player_movement};
use ecs::food::update_food_movement;
use ecs::enemy::{update_ai, update_enemy_movement};
use ecs::particles::{update_particles, drain_particle_queue};
use ecs::globals::dispatch_events;
use resources::{Resources, GamePhase};
use sdl2::pixels::Color;

fn main() {
    let config: GameConfig = fs::read_to_string("config.toml")
        .ok()
        .and_then(|contents| toml::from_str(&contents).ok())
        .unwrap_or_default();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    
    let window = video_subsystem.window("Primordiale - Rust ECS", display_mode.w as u32, display_mode.h as u32)
        .fullscreen_desktop().opengl().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    
    // Keep cursor visible during Menu selection if wanted, hidden for game loop
    sdl_context.mouse().show_cursor(true);
    
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "resources/font.ttf";
    
    // Load discrete sizes to maintain clean retro pixel alignment
    let font = ttf_context.load_font(font_path, 28).expect("Missing resources/font.ttf");
    let title_font = ttf_context.load_font(font_path, 72).expect("Missing resources/font.ttf");

    let mut audio = AudioSystem::new();
    audio.load_sfx("eat", "resources/sfx/eat-1.wav");
    audio.load_sfx("kill", "resources/sfx/kill.wav");
    audio.load_sfx("level", "resources/sfx/level-1.wav");
    audio.play_bgm("resources/bgm/bgm.mp3");

    let timer_subsystem = sdl_context.timer().unwrap();
    let initial_tick = timer_subsystem.ticks();

    let target_fps = config.window.target_fps;
    let shake_duration = config.juice.shake_duration;

    let mut res = Resources::new(display_mode.w as u32, display_mode.h as u32, initial_tick, config);
    let mut world = World::new();

    res.player_entity = Some(spawn_player(&mut world, &res.config));  
    init_level(&mut world, &mut res);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_time = timer_subsystem.ticks();
    
    let time_step = 1.0 / target_fps as f32; 
    let mut accumulator = 0.0f32;

    'running: loop {
        let current_time = timer_subsystem.ticks();
        let frame_time = (current_time - last_time) as f32 / 1000.0;
        last_time = current_time;

        res.timer.fps = if frame_time > 0.0 { 1.0 / frame_time } else { 0.0 };
        res.timer.current_tick = current_time;
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    res.cursor.pos = Vec2::new(x as f32, y as f32);
                }
                Event::MouseButtonDown { .. } => {
                    match res.phase {
                        GamePhase::Menu => {
                            // Clear out menu, drop cursor visibility, jump into active stage
                            res.phase = GamePhase::Active;
                            res.timer.elapsed_time = 0.0;
                            sdl_context.mouse().show_cursor(false);
                        }
                        GamePhase::GameOver => {
                            res.phase = GamePhase::Active;
                            res.progress.current_level = 1;
                            res.progress.highest_level = res.progress.highest_level.max(1);
                            res.progress.food_target = 5;
                            res.progress.enemy_count = 1;
                            res.progress.enemy_speed_modifier = 0.0;
                            res.timer.elapsed_time = 0.0;
                            res.shake.duration = 0;
                            res.is_level_cleared = false;
                            
                            init_level(&mut world, &mut res);
                            sdl_context.mouse().show_cursor(false);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        match res.phase {
            GamePhase::Menu => {
                // Background ticks processing exclusively while keeping game world updates paused
                res.timer.elapsed_time = 0.0;
            }
            GamePhase::Active => {
                accumulator += frame_time.min(0.25);

                while accumulator >= time_step {
                    res.timer.elapsed_time += time_step;
                    let mut cmd = CommandBuffer::new();
                    
                    update_ai(&mut world, &res);
                    update_player_movement(&mut world, &res, time_step);
                    update_food_movement(&mut world, &res, time_step);
                    update_enemy_movement(&mut world, time_step);
                    update_particles(&mut world, &res, time_step, &mut cmd);
                    
                    let mut player_pos = Vec2::zero();
                    let mut player_size = Vec2::zero();
                    if let Some(player) = res.player_entity {
                        if let Ok(tf) = world.get::<&ecs::shared::Transform>(player) {
                            player_pos = tf.pos;
                            player_size = tf.size;
                        }
                    }

                    let intersects = |p1: Vec2, s1: Vec2, p2: Vec2, s2: Vec2| -> bool {
                        p1.x < p2.x + s2.x && p1.x + s1.x > p2.x && p1.y < p2.y + s2.y && p1.y + s1.y > p2.y
                    };

                    for (_, (tf, _)) in world.query_mut::<(&ecs::shared::Transform, &ecs::enemy::Enemy)>() {
                        if intersects(player_pos, player_size, tf.pos, tf.size) {
                            res.events.events.push(ecs::globals::GameEvent::Kill);
                        }
                    }

                    let mut eaten_foods = Vec::new();
                    for (ent, (tf, _f)) in world.query_mut::<(&ecs::shared::Transform, &ecs::food::Food)>() {
                        if intersects(player_pos, player_size, tf.pos, tf.size) {
                            eaten_foods.push((ent, tf.pos));
                        }
                    }

                    let total_food = world.query::<&ecs::food::Food>().iter().count();
                    let ate_count = eaten_foods.len();
                    let is_level_cleared = ate_count > 0 && total_food <= ate_count;

                    for (ent, pos) in eaten_foods {
                        cmd.despawn(ent);
                        res.shake.duration = shake_duration;   
                        res.events.events.push(ecs::globals::GameEvent::Eat(pos));
                    }

                    if is_level_cleared {
                        res.events.events.push(ecs::globals::GameEvent::LevelUp);
                    }

                    dispatch_events(&mut world, &mut res, &audio);
                    drain_particle_queue(&res.config, &mut res.particles, &mut cmd);
                    cmd.run_on(&mut world);

                    if res.shake.duration > 0 {
                        res.shake.duration -= 1;
                    }

                    accumulator -= time_step;
                }

                if res.is_level_cleared {
                    res.is_level_cleared = false;
                    level_up(&mut world, &mut res);
                }
            }
            GamePhase::GameOver => {}
        }

        let total_ms = (res.timer.elapsed_time * 1000.0) as u32;
        res.timer.timer_text = format!("{:02}:{:02}:{:03}", total_ms / 60000, (total_ms / 1000) % 60, total_ms % 1000);

        // Only render the game field if we aren't displaying our title sequence
        if res.phase != GamePhase::Menu {
            render::draw_world(&world, &res, &mut canvas);
        } else {
            canvas.set_draw_color(Color::RGB(12, 12, 14)); // Deep charcoal retro background tint
            canvas.clear();
        }

        // Render UI overlay containing font layers
        render::draw_ui(&res, &mut canvas, &font, &title_font);

        canvas.present();
    }
}