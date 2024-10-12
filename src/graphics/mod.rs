pub mod circles;
pub mod lines;

pub use lines::*;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, video::Window,
    EventPump,
};

const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;
const TITLE: &str = "ROAD INTERSECTION";

pub struct Interface {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub roads: lines::Roads,
}

impl Interface {
    /// This method holds all the logic around the `SDL` implementation.
    /// It initializes the "context" that will be then used to create
    /// a new "window" given a `title` and `dimensions`.
    /// Afterwards it turns the window to a "canvas".
    /// It also initializes a "event pump" from the context
    /// to get the `user` input `events`.
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

        let roads = lines::Roads::new(WIDTH, HEIGHT);

        Ok(Self {
            roads,
            canvas,
            event_pump,
        })
    }

    pub fn running(&mut self) -> Result<(), String> {
        let events = self.event_pump.poll_iter();

        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => return Err("Exiting...".to_string()),

                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => {} // TODO: Generate new `vehicle` from North

                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => {} // TODO: Generate new `vehicle` from West

                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => {} // TODO: Generate new `vehicle` from South

                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => {} // TODO: Generate new `vehicle` from East

                _ => {}
            };
        }

        self.display()
    }

    fn display(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        self.roads
            .vertical
            .iter()
            .for_each(|line| self.canvas.draw_line(line.0, line.1).unwrap());

        self.roads
            .horizontal
            .iter()
            .for_each(|line| self.canvas.draw_line(line.0, line.1).unwrap());

        self.canvas.present();
        Ok(())
    }
}
