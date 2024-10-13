use sdl2::{pixels::Color, render::Canvas, video::Window};

use super::lines::Line;

pub struct Square {
    size: i32,
    sides: Vec<Line>,
    color: Color,
}

impl Square {
    pub fn new(x: i32, y: i32, size: i32, color: Color) -> Self {
        let top = Line::new(x, y, x + size, y);
        let right = Line::new(x + size, y, x + size, y + size);
        let bottom = Line::new(x + size, y + size, x, y + size);
        let left = Line::new(x, y + size, x, y);

        Self {
            color,
            size,
            sides: vec![top, right, bottom, left],
        }
    }

    /// This function performs an iteration
    /// over the road's vertical and horizontal lines
    /// and draw them on the canvas.
    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        for line in self.sides.iter() {
            canvas.draw_line(line.get_start(), line.get_end()).unwrap()
        }

        Ok(())
    }
}
