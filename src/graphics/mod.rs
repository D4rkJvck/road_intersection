mod road;
mod vehicle;

use crate::models::Direction;
use road::Road;
use sdl2::{
    event::Event::{KeyDown, Quit},
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    video::Window,
    EventPump,
};
use std::{thread, time::Duration};
use vehicle::Vehicle;

const TITLE: &str = "ROAD INTERSECTION";
const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;

pub struct Interface {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    road: Road,
    vehicles: Vec<Vehicle>,
}

impl Interface {
    /// This method holds all the logic around the `SDL` implementation.
    /// It initializes the "context" that will be then used to create
    /// a new "window" given a `title` and `dimensions`.
    /// Afterwards it turns the window to a "canvas".
    /// It also initializes a "event pump" from the context
    /// to get the `user` input `events`.
    /// For this specific project, it will also initialize road limits.
    /// It finally initializes the instance of the `interface``.
    pub fn new() -> Result<Self, String> {
        // Initialize the SDL.
        let sdl_ctx = sdl2::init()?;
        let video_subsys = sdl_ctx.video()?;

        let window = video_subsys // Generate the window from the video subsystem
            .window(TITLE, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window // Turn the window into a canvas
            .into_canvas()
            .build()
            .unwrap();

        let event_pump = sdl_ctx // Initialize a event pump to store user inputs
            .event_pump()
            .unwrap();

        let road = Road::new(WIDTH, HEIGHT);
        let vehicles = Vec::new();

        Ok(Self {
            canvas,
            event_pump,
            road,
            vehicles,
        })
    }

    /// This is the core visual function of the program.
    /// It is responsible for updating the state of the
    /// window.
    pub fn running(&mut self) -> Result<(), String> {
        // Remove the vehicules that moved out of the window.
        self.vehicles.retain(|v| v.is_in_window());

        self.render()?;
        self.listen()
    }

    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.road.display(&mut self.canvas)?;

        self.vehicles.iter_mut().for_each(|v| {
            v.check_intersection(&self.road.intersection);
            v.display(&mut self.canvas).unwrap();
        });

        self.canvas.present();

        thread::sleep(Duration::from_millis(16));

        Ok(())
    }

    /// This function will act like a server that will handle
    /// user's input as request to call the regarded functions
    /// as handlers.
    fn listen(&mut self) -> Result<(), String> {
        let events = self.event_pump.poll_iter();

        for event in events {
            match event {
                KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => self.vehicles.push(Vehicle::new(
                    WIDTH as i32 / 2 + 5,
                    HEIGHT as i32 - 45,
                    40,
                    Direction::North,
                    Color::CYAN,
                )),
                KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => self.vehicles.push(Vehicle::new(
                    0,
                    HEIGHT as i32 / 2 + 5,
                    40,
                    Direction::East,
                    Color::CYAN,
                )),
                KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => self.vehicles.push(Vehicle::new(
                    WIDTH as i32 / 2 - 45,
                    0,
                    40,
                    Direction::South,
                    Color::CYAN,
                )),
                KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => self.vehicles.push(Vehicle::new(
                    WIDTH as i32 - 45,
                    HEIGHT as i32 / 2 - 45,
                    40,
                    Direction::West,
                    Color::CYAN,
                )),
                Quit { .. }
                | KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => return Err("Exiting...".to_string()),
                _ => {}
            }
        }

        Ok(())
    }
}
