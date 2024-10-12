pub mod circles;
mod lines;

// use lines::Roads;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump,
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
        self.render()?;
        self.listen()
    }

    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    fn render(&mut self) -> Result<(), String> {
        self.display_roads()?;

        self.canvas.present();

        Ok(())
    }

    /// This function will act like a server that will handle
    /// user's input as request to call the regarded functions
    /// as a handler.
    fn listen(&mut self) -> Result<(), String> {
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

        Ok(())
    }

    /// This function performs an iteration
    /// over the road's vertical and horizontal lines
    /// and draw them on the canvas.
    fn display_roads(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        self.roads.get_vertical().iter().for_each(|line| {
            self.canvas
                .draw_line(line.get_start(), line.get_end())
                .unwrap()
        });

        self.roads.get_horizontal().iter().for_each(|line| {
            self.canvas
                .draw_line(line.get_start(), line.get_end())
                .unwrap()
        });

        Ok(())
    }
}
