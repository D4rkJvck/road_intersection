mod graphics;
mod models;

use models::{Road, Vehicle};
use sdl2::{render::Canvas, video::Window, EventPump};

const TITLE: &str = "ROAD INTERSECTION";
const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;

/// The Interface between the user
/// and the program.
/// Holds all the necessary tools
/// to interact with the user.
pub struct Interface {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    road: Road,
    vehicles: Vec<Vehicle>,
}
