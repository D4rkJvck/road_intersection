use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::models::Direction;

use super::{HEIGHT, WIDTH};

/// This is the only
/// element that will
/// perform translation
/// updates.
pub struct Vehicle {
    rect: Rect,
    direction: Direction,
    speed: i32,
    color: Color,
    turned_left: bool,
}

impl Vehicle {
    /// The structure is initialized using the sdl2 Rect structure's parameters and
    /// some additional such as the color and the direction that will be pretty useful.
    /// The rest of the structure's fields will be calculated from the given parameters
    /// or just initialize to a default value.
    pub fn new(x: i32, y: i32, side: u32, direction: Direction, color: Color) -> Self {
        Self {
            rect: Rect::new(x, y, side, side),
            direction,
            speed: 1,
            color,
            turned_left: false,
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

    pub fn check_intersection(&mut self, intersection: &Rect) {
        use Direction::*;

        match (&mut self.direction, &self.color) {
            // Right Turner
            (North, &Color::YELLOW) => {
                if self.rect.bottom() == intersection.bottom() - 5 {
                    self.direction = East;
                }
            }
            (East, &Color::YELLOW) => {
                if self.rect.left() == intersection.left() + 5 {
                    self.direction = South;
                }
            }
            (South, &Color::YELLOW) => {
                if self.rect.top() == intersection.top() + 5 {
                    self.direction = West;
                }
            }
            (West, &Color::YELLOW) => {
                if self.rect.right() == intersection.right() - 5 {
                    self.direction = North;
                }
            }

            // Left Turner
            //BUG Logical cycling... // TEMP: Fixed...
            (North, &Color::CYAN) => {
                if self.rect.top() == intersection.top() + 5 && !self.turned_left {
                    self.direction = West;
                    self.turned_left = true;
                }
            }
            (East, &Color::CYAN) => {
                if self.rect.right() == intersection.right() - 5 && !self.turned_left {
                    self.direction = North;
                    self.turned_left = true;
                }
            }
            (South, &Color::CYAN) => {
                if self.rect.bottom() == intersection.bottom() - 5 && !self.turned_left {
                    self.direction = East;
                    self.turned_left = true;
                }
            }
            (West, &Color::CYAN) => {
                if self.rect.left() == intersection.left() + 5 && !self.turned_left {
                    self.direction = South;
                    self.turned_left = true;
                }
            }
            _ => {}
        };

        //todo:(check if the light is green before to set the speed to zero)
        // if self.top == intersection.bottom() as u32 {
        //     self.speed = 0;
        //     // return true;
        // }
        // false
    }

    /// This function is crucial when
    /// it comes to remove vehicule
    /// as it confirm that the vehicle is
    /// out of the window...
    pub fn is_in_window(&self) -> bool {
        self.rect.top() < HEIGHT as i32
            && self.rect.left() < WIDTH as i32
            && self.rect.right() > 0
            && self.rect.bottom() > 0
    }
}
