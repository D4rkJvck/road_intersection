use sdl2::{pixels::Color, rect::Point};

use super::lines::Line;

pub struct VehicleArea {
    color: Color,
    position: Point,
    sides: Vec<Line>
}

impl VehicleArea {
    pub fn new(color: Color, x: i32, y: i32) -> Self {
        let position = Point::new(x, y);

        let top = Line::new(x, y, x + 50, y );
        let right = Line::new(x + 50, y, x + 50, y + 50);
        let bottom = Line::new(x + 50, y + 50, x, y + 50);
        let left = Line::new(x, y + 50, x, y);

        Self {
            color,
            position,
            sides: vec![top, right, bottom, left],
        }
    }

    pub fn get_sides(&self) -> &Vec<Line> {
        &self.sides
    }
}