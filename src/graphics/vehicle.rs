use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::models::Direction;

const SPEED: i32 = 1;

pub struct Vehicle {
    rect: Rect,
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
    direction: Direction,
    color: Color,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, side: u32, direction: Direction, color: Color) -> Self {
        Self {
            rect: Rect::new(x, y, side, side),
            top: y,
            right: x + side as i32,
            bottom: y + side as i32,
            left: x,
            direction,
            color,
        }
    }

    /// This function performs an iteration
    /// over the road's vertical and horizontal lines
    /// and draw them on the canvas.
    pub fn display(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        match self.direction {
            Direction::North => self.rect.y -= SPEED,
            Direction::East => self.rect.x += SPEED,
            Direction::South => self.rect.y += SPEED,
            Direction::West => self.rect.x -= SPEED,
        }

        canvas.fill_rect(self.rect)?;

        Ok(())
    }

    // pub fn get_front(&self) -> u32
}
