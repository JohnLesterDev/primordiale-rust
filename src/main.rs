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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    
    let window = video_subsystem.window("Primordiale - Rust ECS", display_mode.w as u32, display_mode.h as u32)
        .fullscreen_desktop().opengl().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    sdl_context.mouse().show_cursor(false);
    
    // Initialize TrueType Fonts
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "resources/font.ttf"; // CRITICAL: This file MUST exist.
    let font = ttf_context.load_font(font_path, 45).expect("Missing resources/font.ttf");

    let mut audio = AudioSystem::new();
    audio.load_sfx("eat", "resources/sfx/eat-1.wav");
    audio.load_sfx("kill", "resources/sfx/kill.wav");
    audio.load_sfx("level", "resources/sfx/level-1.wav");
    AudioSystem::play_bgm("resources/bgm/bgm.mp3");

    let timer_subsystem = sdl_context.timer().unwrap();
    let initial_tick = timer_subsystem.ticks();

    let mut state = GameState {
        display_width: display_mode.w as u32, display_height: display_mode.h as u32,
        current_level: 1, highest_level: 1, food_target: 5, enemy_count: 1,
        is_game_over: false, 
        start_time: initial_tick, 
        current_tick: initial_tick,
        elapsed_time: 0.0, // Initialized
        shake: 0, shake_offset: Vec2::zero(),
        mouse_pos: Vec2::new(display_mode.w as f32 / 2.0, display_mode.h as f32 / 2.0),
        timer_text: String::new(), enemy_speed_modifier: 0.0, fps: 0.0,
    };

    let mut world = World::new();
    let pw = PLAYER_DIMEN.0 * state.display_height as f32;
    let ph = PLAYER_DIMEN.1 * state.display_height as f32;
    world.spawn((
        Transform { pos: Vec2::new(0.0, 0.0), size: Vec2::new(pw, ph) },
        Renderable { color: (106, 109, 115) }, Player
    ));

    init_level(&mut world, &mut state);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_tick = timer_subsystem.ticks();

    'running: loop {
        let current_tick = timer_subsystem.ticks();
        state.current_tick = current_tick; 
        let dt = (current_tick - last_tick) as f32 / 1000.0;
        last_tick = current_tick;

        state.fps = if dt > 0.0 { 1.0 / dt } else { 0.0 };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { x, y, .. } => state.mouse_pos = Vec2::new(x as f32, y as f32),
                Event::MouseButtonDown { .. } if state.is_game_over => {
                    state.is_game_over = false;
                    state.current_level = 1; state.food_target = 8; state.enemy_count = 2; 
                    state.enemy_speed_modifier = 0.0; state.elapsed_time = 0.0; // Reset Time
                    init_level(&mut world, &mut state);
                    sdl_context.mouse().show_cursor(false);
                }
                _ => {}
            }
        }

        if !state.is_game_over {
            state.elapsed_time += dt; // Freeze logical timer on death
            
            let mut cmd = CommandBuffer::new();
            update_ai(&mut world, &mut state, dt);
            update_movement(&mut world, &state, dt, &mut cmd);
            check_collisions(&mut world, &mut state, &audio, &mut cmd);
            cmd.run_on(&mut world);
            
            render(&mut world, &mut state, &mut canvas);
        } else {
            sdl_context.mouse().show_cursor(true);
            canvas.set_draw_color(Color::RGB(0, 0, 0)); 
            canvas.clear(); 
        }

        // --- HUD & Text Rendering Pipeline ---
        let total_ms = (state.elapsed_time * 1000.0) as u32;
        state.timer_text = format!("{:02}:{:02}:{:03}", total_ms / 60000, (total_ms / 1000) % 60, total_ms % 1000);

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

        if !state.is_game_over {
            draw_text(&format!("FPS: {:.2}", state.fps), 10, 10, false);
            draw_text(&format!("Level: {}", state.current_level), state.display_width as i32 - 150, 10, false);
            draw_text(&state.timer_text, (state.display_width / 2) as i32, 10, true);
        } else {
            let cx = (state.display_width / 2) as i32;
            let cy = (state.display_height / 2) as i32;
            draw_text("Game Over", cx, cy - 50, true);
            draw_text(&format!("Highest Level: {} On {}", state.highest_level, state.timer_text), cx, cy, true);
            draw_text("Click to Restart", cx, cy + 50, true);
        }

        canvas.present();
    }
}