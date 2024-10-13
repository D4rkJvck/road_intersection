mod road;
mod vehicle;

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};
use std::collections::HashMap;

pub enum Direction {
    North,
    East,
    South,
    West,
}

/// Junction between two points.
/// This will useful as it facilitates
/// the creation of more complex models
/// such as the road lines.
pub struct Line(Point, Point);

impl Line {
    /// As arguments, the new function will take the x and y
    /// coordinates of the two points that compose the line.
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self(Point::new(x1, y1), Point::new(x2, y2))
    }
}

/// This structure represents the map
/// onto which the simulation will
/// happen. It basically contains most of the
/// objects to be visualized in the simulation.
pub struct Road {
    lines: Vec<Line>,
    pub intersection: Rect,
    traffic_lights: HashMap<String, Rect>,
}

/// This is the only
/// element that will
/// perform translation
/// updates.
pub struct Vehicle {
    rect: Rect,
    direction: Direction,
    speed: i32,
    color: Color,
    cross: bool,
}
