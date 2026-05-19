// ==> Rewrite src/render/ui.rs <==
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use crate::resources::{Resources, GamePhase};

pub fn draw_ui(
    res: &Resources,
    canvas: &mut WindowCanvas,
    font: &Font,          // Main game font (e.g., 45px)
    title_font: &Font,    // Larger font target for titles (e.g., 90px)
) {
    let texture_creator = canvas.texture_creator();
    
    // Abstracted inline helper for uniform font rendering across phases
    let mut draw_text_ext = |text: &str, x: i32, y: i32, center: bool, color: Color, target_font: &Font| {
        if let Ok(surface) = target_font.render(text).blended(color) {
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

    let cx = (res.display.width / 2) as i32;
    let cy = (res.display.height / 2) as i32;

    match res.phase {
        GamePhase::Menu => {
            // Title Header with Drop Shadow
            draw_text_ext("PRIMORDIALE", cx + 4, cy - 116, true, Color::RGB(40, 40, 45), title_font);
            draw_text_ext("PRIMORDIALE", cx, cy - 120, true, Color::RGB(189, 25, 23), title_font);

            // Subtitle
            draw_text_ext("Devour them all!", cx, cy - 50, true, Color::RGB(106, 109, 115), font);

            // Flashing Prompt via Sinusoidal Alpha Modulation
            let ticks = res.timer.current_tick;
            let flash_condition = (ticks / 400) % 2 == 0; 
            
            if flash_condition {
                draw_text_ext("PRESS START", cx, cy + 80, true, Color::RGB(255, 255, 255), font);
            }

            // Technical details footer
            draw_text_ext("CLICK MOUSE TO INITIALIZE", cx, res.display.height as i32 - 60, true, Color::RGB(82, 163, 65), font);
        }
        GamePhase::Active => {
            draw_text_ext(&format!("FPS: {:.0}", res.timer.fps), 10, 10, false, Color::RGB(255, 255, 255), font);
            draw_text_ext(&format!("Level: {}", res.progress.current_level), res.display.width as i32 - 250, 10, false, Color::RGB(255, 255, 255), font);
            draw_text_ext(&res.timer.timer_text, cx, 10, true, Color::RGB(255, 255, 255), font);
        }
        GamePhase::GameOver => {
            draw_text_ext("Game Over", cx, cy - 50, true, Color::RGB(189, 25, 23), title_font);
            draw_text_ext(&format!("Highest Level: {} On {}", res.progress.highest_level, res.timer.timer_text), cx, cy, true, Color::RGB(255, 255, 255), font);
            draw_text_ext("Click to Restart", cx, cy + 60, true, Color::RGB(106, 109, 115), font);
        }
    }
}