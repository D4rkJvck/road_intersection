use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::models::Direction;

pub struct Square {
    rect: Rect,
    front: i32,
    color: Color,
}

impl Square {
    pub fn new(x: i32, y: i32, side: u32, direction: Direction, color: Color) -> Self {
        let rect = Rect::new(x, y, side, side);
        let front = match direction {
            Direction::North => y,
            Direction::East => x + side as i32,
            Direction::South => y + side as i32,
            Direction::West => x,
        };

        Self { rect, front, color }
    }

    /// This function performs an iteration
    /// over the road's vertical and horizontal lines
    /// and draw them on the canvas.
    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        canvas.fill_rect(self.rect)?;

        Ok(())
    }
}
