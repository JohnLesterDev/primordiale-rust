// ==> src/main.rs <==
mod resources;
mod engine;
mod ecs;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use hecs::{World, CommandBuffer};

use engine::settings::*;
use engine::math::Vec2;
use engine::audio::AudioSystem;
use ecs::components::*;
use ecs::resources::GameState;
use ecs::systems::*;
use ecs::globals::dispatch_events; // Pulled dispatcher clean into scope
use resources::{Resources, GamePhase};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    
    let window = video_subsystem.window("Primordiale - Rust ECS", display_mode.w as u32, display_mode.h as u32)
        .fullscreen_desktop().opengl().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    sdl_context.mouse().show_cursor(false);
    
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "resources/font.ttf";
    let font = ttf_context.load_font(font_path, 45).expect("Missing resources/font.ttf");

    let mut audio = AudioSystem::new();
    audio.load_sfx("eat", "resources/sfx/eat-1.wav");
    audio.load_sfx("kill", "resources/sfx/kill.wav");
    audio.load_sfx("level", "resources/sfx/level-1.wav");
    audio.play_bgm("resources/bgm/bgm.mp3");

    let timer_subsystem = sdl_context.timer().unwrap();
    let initial_tick = timer_subsystem.ticks();

    // 1. Instantiate the centralized Resources container
    let mut res = Resources::new(display_mode.w as u32, display_mode.h as u32, initial_tick);

    // Legacy State kept alive for subsystem dependencies during Phase 1 transition.
    let mut state = GameState {
        display_width: display_mode.w as u32, display_height: display_mode.h as u32,
        current_level: 1, highest_level: 1, food_target: 5, enemy_count: 1,
        is_game_over: false, start_time: initial_tick, current_tick: initial_tick,
        elapsed_time: 0.0, shake: 0, shake_offset: Vec2::zero(),
        mouse_pos: Vec2::new(display_mode.w as f32 / 2.0, display_mode.h as f32 / 2.0),
        timer_text: String::new(), enemy_speed_modifier: 0.0, fps: 0.0,
        events: Vec::new(),
    };

    let mut world = World::new();
    let pw = PLAYER_DIMEN.0;
    let ph = PLAYER_DIMEN.1;
    
    world.spawn((
        Transform { pos: Vec2::new(0.0, 0.0), size: Vec2::new(pw, ph) },
        Renderable { color: (106, 109, 115) }, Player
    ));

    init_level(&mut world, &mut state);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_time = timer_subsystem.ticks();
    
    let time_step = 1.0 / FPS as f32; 
    let mut accumulator = 0.0f32;

    'running: loop {
        let current_time = timer_subsystem.ticks();
        let frame_time = (current_time - last_time) as f32 / 1000.0;
        last_time = current_time;

        res.timer.current_tick = current_time;
        state.current_tick = current_time;
        
        res.timer.fps = if frame_time > 0.0 { 1.0 / frame_time } else { 0.0 };
        state.fps = res.timer.fps;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    res.cursor.pos = Vec2::new(x as f32, y as f32);
                    state.mouse_pos = res.cursor.pos;
                }
                Event::MouseButtonDown { .. } => {
                    if res.phase == GamePhase::GameOver {
                        res.phase = GamePhase::Active;
                        res.progress.current_level = 1;
                        res.progress.food_target = 8;
                        res.progress.enemy_count = 2;
                        res.progress.enemy_speed_modifier = 0.0;
                        res.timer.elapsed_time = 0.0;
                        res.shake.duration = 0;
                        res.shake.offset = Vec2::zero();

                        state.is_game_over = false;
                        state.current_level = 1; state.food_target = 8; state.enemy_count = 2; 
                        state.enemy_speed_modifier = 0.0; state.elapsed_time = 0.0;
                        state.shake = 0; state.shake_offset = Vec2::zero();
                        
                        init_level(&mut world, &mut state);
                        sdl_context.mouse().show_cursor(false);
                    }
                }
                _ => {}
            }
        }

        match res.phase {
            GamePhase::Active => {
                accumulator += frame_time.min(0.25);

                while accumulator >= time_step {
                    res.timer.elapsed_time += time_step;
                    state.elapsed_time = res.timer.elapsed_time;
                    
                    let mut cmd = CommandBuffer::new();
                    update_ai(&mut world, &mut state, time_step);
                    update_movement(&mut world, &state, time_step, &mut cmd);
                    check_collisions(&mut world, &mut state, &mut cmd);
                    
                    if state.is_game_over {
                        res.phase = GamePhase::GameOver;
                    }

                    if !state.events.is_empty() {
                        res.events.events.extend(state.events.drain(..));
                    }

                    // Pass mutable state handle into the dispatcher pipeline
                    dispatch_events(&mut world, &mut res, &mut state, &audio, &mut cmd);

                    cmd.run_on(&mut world);

                    if state.shake > 0 {
                        state.shake -= 1;
                        res.shake.duration = state.shake;
                    }

                    accumulator -= time_step;
                }
            }
            GamePhase::GameOver => {}
        }

        // --- Render Pipeline ---
        draw_world(&mut world, &state, &mut canvas);

        // --- Stateless UI Pipeline ---
        let total_ms = (res.timer.elapsed_time * 1000.0) as u32;
        res.timer.timer_text = format!("{:02}:{:02}:{:03}", total_ms / 60000, (total_ms / 1000) % 60, total_ms % 1000);
        state.timer_text = res.timer.timer_text.clone();

        let texture_creator = canvas.texture_creator();
        let mut draw_text = |text: &str, x: i32, y: i32, center: bool| {
            if let Ok(surface) = font.render(text).blended(Color::RGB(255, 255, 255)) {
                if let Ok(texture) = texture_creator.create_texture_from_surface(&surface) {
                    let mut rect = Rect::new(x, y, surface.width(), surface.height());
                    if center { 
                        rect.x -= (surface.width() / 2) as i32; 
                        rect.y -= (surface.height() / 2) as i32; 
                    }
                    canvas.copy(&texture, None, rect).ok();
                }
            }
        };

        match res.phase {
            GamePhase::Active => {
                draw_text(&format!("FPS: {:.2}", res.timer.fps), 10, 10, false);
                draw_text(&format!("Level: {}", state.current_level), res.display.width as i32 - 250, 10, false);
                draw_text(&res.timer.timer_text, (res.display.width / 2) as i32, 10, true);
            }
            GamePhase::GameOver => {
                let cx = (res.display.width / 2) as i32;
                let cy = (res.display.height / 2) as i32;
                draw_text("Game Over", cx, cy - 50, true);
                draw_text(&format!("Highest Level: {} On {}", state.highest_level, res.timer.timer_text), cx, cy, true);
                draw_text("Click to Restart", cx, cy + 50, true);
            }
        }

        canvas.present();
    }
}