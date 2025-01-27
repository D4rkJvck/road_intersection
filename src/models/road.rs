use super::{Line, Road};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::collections::HashMap;

impl Road {
    /// The structure will be initialized using
    /// the window dimensions. It will generate
    /// and store every static element in terms
    /// of translation including the road lines
    /// that it generates using the Line structure.
    pub fn new(width: u32, height: u32) -> Self {
        let mid_height = height as i32 / 2;
        let mid_width = width as i32 / 2;

        let lines = vec![
            // Horizontal lines
            Line::new(0, mid_height - 50, width as i32, mid_height - 50), // Top
            Line::new(0, mid_height, width as i32, mid_height),           // Middle
            Line::new(0, mid_height + 50, width as i32, mid_height + 50), // Bottom
            // Vertical lines
            Line::new(mid_width - 50, 0, mid_width - 50, height as i32), // Left
            Line::new(mid_width, 0, mid_width, height as i32),           // Center
            Line::new(mid_width + 50, 0, mid_width + 50, height as i32), // Right
        ];

        let intersection = Rect::new(mid_width - 50, mid_height - 50, 101, 101);

        let mut traffic_lights = HashMap::new();
        traffic_lights.insert(
            "North".to_string(),
            Rect::new(mid_width + 60, mid_height + 60, 25, 25),
        );
        traffic_lights.insert(
            "South".to_string(),
            Rect::new(mid_width - 85, mid_height - 85, 25, 25),
        );
        traffic_lights.insert(
            "West".to_string(),
            Rect::new(mid_width + 60, mid_height - 85, 25, 25),
        );
        traffic_lights.insert(
            "East".to_string(),
            Rect::new(mid_width - 85, mid_height + 60, 25, 25),
        );

        Self {
            lines,
            intersection,
            traffic_lights,
        }
    }

    /// This function performs an iteration
    /// over the road's lines, the intersection
    /// and the traffic lights. It will then
    /// draw them on the canvas.
    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::WHITE);

        // Draw 3 vertical and 3 horizontal lines
        // in the middle of the window
        for line in self.lines.iter() {
            canvas.draw_line(line.0, line.1)?;
        }

        // Draw the road intersection
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(self.intersection)?;

        // Draw the traffic lights
        canvas.set_draw_color(Color::RED);
        for (_, light) in self.traffic_lights.iter() {
            canvas.fill_rect(*light)?;
        }

        Ok(())
    }
}
