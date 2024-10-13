use sdl2::{pixels::Color, render::Canvas, video::Window};

use super::lines::Line;

pub struct VehicleArea {
    color: Color,
    sides: Vec<Line>,
}

impl VehicleArea {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        let top = Line::new(x, y, x + 50, y);
        let right = Line::new(x + 50, y, x + 50, y + 50);
        let bottom = Line::new(x + 50, y + 50, x, y + 50);
        let left = Line::new(x, y + 50, x, y);

        Self {
            color,
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
