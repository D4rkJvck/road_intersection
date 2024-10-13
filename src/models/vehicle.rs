use super::Vehicle;
use crate::{models::Direction, HEIGHT, WIDTH};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

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
            cross: false,
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

    /// This functions is the traffic monitor.
    /// It role is to manage the vehicle's behaviour depending on
    /// the traffic lights and the other vehicles.
    /// It updates the direction when the vehicle reaches
    /// the intersection based on its previous direction and its color.
    pub fn check_intersection(&mut self, intersection: &Rect) {
        match &self.color {
            &Color::YELLOW => self.turn_right(intersection),
            &Color::CYAN => self.turn_left(intersection),
            _ => {}
        };

        //todo:(check if the light is green before to set the speed to zero)
        // if self.top == intersection.bottom() as u32 {
        //     self.speed = 0;
        //     // return true;
        // }
        // false
    }

    /// It handles the direction assignment of the vehicle
    /// to its right depending on its direction.
    fn turn_right(&mut self, intersection: &Rect) {
        match &self.direction {
            Direction::North => {
                if self.rect.bottom() == intersection.bottom() - 5 {
                    self.direction = Direction::East;
                    self.cross = true;
                }
            }
            Direction::East => {
                if self.rect.left() == intersection.left() + 5 {
                    self.direction = Direction::South;
                    self.cross = true;
                }
            }
            Direction::South => {
                if self.rect.top() == intersection.top() + 5 {
                    self.direction = Direction::West;
                    self.cross = true;
                }
            }
            Direction::West => {
                if self.rect.right() == intersection.right() - 5 {
                    self.direction = Direction::North;
                    self.cross = true;
                }
            }
        }
    }

    /// It handles the direction assignment of the vehicle
    /// to its left depending on its direction.
    fn turn_left(&mut self, intersection: &Rect) {
        match &self.direction {
            Direction::North => {
                if self.rect.top() == intersection.top() + 5 && !self.cross {
                    self.direction = Direction::West;
                    self.cross = true;
                }
            }
            Direction::East => {
                if self.rect.right() == intersection.right() - 5 && !self.cross {
                    self.direction = Direction::North;
                    self.cross = true;
                }
            }
            Direction::South => {
                if self.rect.bottom() == intersection.bottom() - 5 && !self.cross {
                    self.direction = Direction::East;
                    self.cross = true;
                }
            }
            Direction::West => {
                if self.rect.left() == intersection.left() + 5 && !self.cross {
                    self.direction = Direction::South;
                    self.cross = true;
                }
            }
        }
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
