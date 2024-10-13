use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::models::Direction;

use super::{HEIGHT, WIDTH};

/// This is the only
/// element that will
/// perform translation
/// updates.
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
    /// The structure is initialized using the sdl2 Rect structure's parameters and
    /// some additional such as the color and the direction that will be pretty useful.
    /// The rest of the structure's fields will be calculated from the given parameters
    /// or just initialize to a default value.
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

    /// This is the function
    /// responsible for the
    /// translation by
    /// updating the position.
    fn moving(&mut self) {
        match self.direction {
            Direction::North => self.rect.y -= self.speed,
            Direction::East => self.rect.x += self.speed,
            Direction::South => self.rect.y += self.speed,
            Direction::West => self.rect.x -= self.speed,
        };
        self.update_position();
    }

    /// This function unique role is to update
    /// the direction when the vehicle reaches
    /// the intersection based on its previous
    /// direction and its color.
    fn turn(&mut self, direction: Direction) {
        self.direction = direction;
        self.moving();
    }

    /// This function's single
    /// responsibility if to
    /// toggle the movement
    /// of the vehicle.
    fn start_stop(&mut self) {
        match self.speed {
            0 => self.speed = 1,
            _ => self.speed = 0,
        }
    }

    /// This function will update
    /// the position's markers
    /// such as top, left...
    /// whenever the position changes
    fn update_position(&mut self) {
        self.top = self.rect.y as u32;
        self.right = (self.rect.x + self.rect.w) as u32;
        self.bottom = (self.rect.y + self.rect.h) as u32;
        self.left = self.rect.x as u32
    }

    pub fn check_intersection(&mut self, intersection: &Rect) /*-> bool*/
    {
        if let Direction::North = self.direction {
            if let Color::GREEN = self.color {
                if self.top == intersection.top() as u32 + 5 {
                    self.direction = Direction::West;
                }
            }

            //todo:(check if the light is green before to set the speed to zero)
            // if self.top == intersection.bottom() as u32 {
            //     self.speed = 0;
            //     // return true;
            // }
        }
        // false
    }

    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the vehicle is
    /// out of the window...
    pub fn is_in_window(&self) -> bool {
        self.top < HEIGHT && self.left < WIDTH && self.right > 0 && self.bottom > 0
    }
}
