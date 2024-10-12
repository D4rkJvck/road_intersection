use std::{error::Error, process};

use road_intersection::graphics::Interface;

/// Make sure to have minimum responsibility
/// while having an essential responsibility.
/// Responsible for exiting the program when needed.
fn main() -> Result<(), Box<dyn Error>> {
    // Create the window.
    let mut window = Interface::new()?;

    loop {
        if let Err(_) = window.running() {
            process::exit(0)
        }
    }
}
