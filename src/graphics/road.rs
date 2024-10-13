use std::collections::HashMap;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::models::Direction;

use super::squares::Square;

pub struct Line(Point, Point);

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self(Point::new(x1, y1), Point::new(x2, y2))
    }

    pub fn get_start(&self) -> Point {
        self.0
    }

    pub fn get_end(&self) -> Point {
        self.1
    }
}

pub struct Road {
    lines: Vec<Line>,
    intersection: Square,
    // traffic_lights: HashMap<String, Square>,
}

impl Road {
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

        let intersection = Square::new(mid_width - 50, mid_height - 50, 101, Direction::South, Color::RGB(0, 0, 0));

        // let mut traffic_lights = HashMap::new();
        // traffic_lights.insert("North".to_string(),Square::new(mid_width + 60, mid_height + 60, 25, Color::RGB(255, 0, 0)));
        // traffic_lights.insert("South".to_string(), Square::new(mid_width - 85, mid_height - 85, 25, Color::RGB(255, 0, 0)));
        // traffic_lights.insert("West".to_string(),Square::new(mid_width + 60, mid_height - 85, 25, Color::RGB(255, 0, 0)));
        // traffic_lights.insert("East".to_string(),Square::new(mid_width - 85, mid_height + 60, 25, Color::RGB(255, 0, 0)));

        Self {
            lines,
            intersection,
            // traffic_lights,
        }
    }

    /// This function performs an iteration
    /// over the road's lines
    /// and draw them on the canvas.
    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Draw 3 vertical and 3 horizontal lines
        // in the middle of the window
        for line in self.lines.iter() {
            canvas.draw_line(line.0, line.1)?;
        }

        self.intersection.display(canvas)?;

        // for (_, light) in self.traffic_lights.iter() {
        //     light.display(canvas)?;
        // }

        Ok(())
    }
}
