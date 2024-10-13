mod lines;
mod squares;
mod circles;

use lines::RoadLines;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump,
};

const TITLE: &str = "ROAD INTERSECTION";
const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;

pub struct Interface {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    roads: RoadLines,
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

        let roads = RoadLines::new(WIDTH, HEIGHT);

        Ok(Self {
            roads,
            canvas,
            event_pump,
        })
    }

    /// This is the core visual function of the program.
    /// It is responsible for updating the state of the
    /// window.
    pub fn running(&mut self) -> Result<(), String> {
        self.render()?;
        self.listen()
    }

    /// This function is responsible for rendering
    /// everything that has been drawn on the canvas
    /// by calling the concerned drawing functions.
    fn render(&mut self) -> Result<(), String> {
        self.display_road_lines()?;
        self.display_vehicule_area()?;
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
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => {} // TODO: Generate new `vehicle` from "North"
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => {} // TODO: Generate new `vehicle` from "West"
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => {} // TODO: Generate new `vehicle` from "South"
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => {} // TODO: Generate new `vehicle` from "East"
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::ESCAPE), .. } => return Err("Exiting...".to_string()),
                _ => {}
            };
        }

        Ok(())
    }

    /// This function performs an iteration
    /// over the road's vertical and horizontal lines
    /// and draw them on the canvas.
    fn display_road_lines(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Draw 3 vertical lines in the middle of the window
        self.roads.get_vertical().iter().for_each(|line| {
            self.canvas
                .draw_line(line.get_start(), line.get_end())
                .unwrap()
        });

        // Draw 3 horizontal lines in the middle of the window
        self.roads.get_horizontal().iter().for_each(|line| {
            self.canvas
                .draw_line(line.get_start(), line.get_end())
                .unwrap()
        });

        Ok(())
    }

    fn display_vehicule_area(&mut self) -> Result<(), String> {
        let color = Color::RGB(0, 0, 255);
        let square = squares::VehicleArea::new(color, (WIDTH as i32 / 2) - 50, 100);

        self.canvas.set_draw_color(color);

        square.get_sides().iter().for_each(|line| {
            self.canvas
               .draw_line(line.get_start(), line.get_end())
               .unwrap()
        });

        Ok(())
    }
}
