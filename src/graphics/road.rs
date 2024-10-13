use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use std::collections::HashMap;

/// Junction between two points.
/// This will useful as it facilitates
/// the creation of more complex models
/// such as the road lines.
pub struct Line(Point, Point);

impl Line {
    /// As arguments, the new function will take the x and y
    /// coordinates of the two points that compose the line.
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self(Point::new(x1, y1), Point::new(x2, y2))
    }
}

/// This structure represents the map
/// onto which the simulation will
/// happen. It basically contains most of the
/// objects to be visualized in the simulation.
pub struct Road {
    lines: Vec<Line>,
    intersection: Rect,
    traffic_lights: HashMap<String, Rect>,
}

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
