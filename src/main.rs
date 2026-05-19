mod resources;
mod engine;
mod ecs;
mod render;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use hecs::{World, CommandBuffer};

use engine::settings::*;
use engine::math::Vec2;
use engine::audio::AudioSystem;
use ecs::components::*;
use ecs::resources::GameState;
use ecs::systems::*;
use ecs::globals::dispatch_events;
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

    let mut res = Resources::new(display_mode.w as u32, display_mode.h as u32, initial_tick);

    let mut state = GameState {
        display_width: display_mode.w as u32,
        display_height: display_mode.h as u32,
        current_level: 1,
        highest_level: 1,
        food_target: 5,
        enemy_count: 1,
        is_game_over: false,
        start_time: initial_tick,
        current_tick: initial_tick,
        elapsed_time: 0.0,
        shake: 0,
        shake_offset: Vec2::zero(),
        mouse_pos: Vec2::new(display_mode.w as f32 / 2.0, display_mode.h as f32 / 2.0),
        timer_text: String::new(),
        enemy_speed_modifier: 0.0,
        fps: 0.0,
        events: Vec::new(),
    };

    let mut world = World::new();
    let pw = PLAYER_DIMEN.0;
    let ph = PLAYER_DIMEN.1;
    
    world.spawn((
        Transform { pos: Vec2::new(0.0, 0.0), size: Vec2::new(pw, ph) },
        Renderable { color: (106, 109, 115) },
        Player
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
                        // Synchronize state containers on game restart
                        res.phase = GamePhase::Active;
                        res.progress.current_level = 1;
                        res.progress.food_target = 5;
                        res.progress.enemy_count = 1;
                        res.progress.enemy_speed_modifier = 0.0;
                        res.timer.elapsed_time = 0.0;
                        res.shake.duration = 0;
                        res.shake.offset = Vec2::zero();

                        state.is_game_over = false;
                        state.current_level = 1; 
                        state.food_target = 5; 
                        state.enemy_count = 1; 
                        state.enemy_speed_modifier = 0.0; 
                        state.elapsed_time = 0.0;
                        state.shake = 0; 
                        state.shake_offset = Vec2::zero();
                        
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

                    // Pipe legacy collision events seamlessly to central engine dispatcher
                    if !state.events.is_empty() {
                        res.events.events.extend(state.events.drain(..));
                    }

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

        // Generate typography timestamp text representation 
        let total_ms = (res.timer.elapsed_time * 1000.0) as u32;
        res.timer.timer_text = format!("{:02}:{:02}:{:03}", total_ms / 60000, (total_ms / 1000) % 60, total_ms % 1000);
        state.timer_text = res.timer.timer_text.clone();

        // Execution of detached rendering pipelines
        render::draw_world(&world, &res.display, &res.shake, res.phase, &mut canvas);
        render::draw_ui(&res, &state, &mut canvas, &font);

        canvas.present();
    }
}