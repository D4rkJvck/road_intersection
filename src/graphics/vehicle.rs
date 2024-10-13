use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::models::Direction;

use super::{HEIGHT, WIDTH};

#[derive(Debug)]
pub struct Vehicle {
    rect: Rect,
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    direction: Direction,
    speed: i32,
    color: Color,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, side: u32, direction: Direction, color: Color) -> Self {
        Self {
            rect: Rect::new(x, y, side, side),
            top: y as u32,
            right: x as u32 + side,
            bottom: y as u32 + side,
            left: x as u32,
            direction,
            speed: 1,
            color,
        }
    }

    /// This function performs an iteration
    /// over the road's vertical and horizontal lines
    /// and draw them on the canvas.
    pub fn display(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        self.moving();

        canvas.fill_rect(self.rect)?;

        Ok(())
    }

    fn moving(&mut self) {
        match self.direction {
            Direction::North => self.rect.y -= self.speed,
            Direction::East => self.rect.x += self.speed,
            Direction::South => self.rect.y += self.speed,
            Direction::West => self.rect.x -= self.speed,
        };

        self.update_position();
    }

    fn turn(&mut self, direction: Direction) {
        self.direction = direction;
        self.moving();
    }

    fn start_stop(&mut self) {
        match self.speed {
            0 => self.speed = 1,
            _ => self.speed = 0,
        }
    }

    fn update_position(&mut self) {
        self.top = self.rect.y as u32;
        self.right = (self.rect.x + self.rect.w) as u32;
        self.bottom = (self.rect.y + self.rect.h) as u32;
        self.left = self.rect.x as u32
    }

    pub fn is_in_window(&self) -> bool {
        self.top < HEIGHT && self.left < WIDTH && self.right > 0 && self.bottom > 0
    }
}
