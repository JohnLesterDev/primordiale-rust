use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use crate::ecs::resources::GameState;
use crate::resources::{Resources, GamePhase};

pub fn draw_ui(
    res: &Resources,
    state: &GameState,
    canvas: &mut WindowCanvas,
    font: &Font,
) {
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
            // Draw standard game HUD elements
            draw_text(&format!("FPS: {:.2}", res.timer.fps), 10, 10, false);
            draw_text(&format!("Level: {}", state.current_level), res.display.width as i32 - 250, 10, false);
            draw_text(&res.timer.timer_text, (res.display.width / 2) as i32, 10, true);
        }
        GamePhase::GameOver => {
            // Draw centered Game Over layout overlay
            let cx = (res.display.width / 2) as i32;
            let cy = (res.display.height / 2) as i32;
            draw_text("Game Over", cx, cy - 50, true);
            draw_text(&format!("Highest Level: {} On {}", state.highest_level, res.timer.timer_text), cx, cy, true);
            draw_text("Click to Restart", cx, cy + 50, true);
        }
    }
}