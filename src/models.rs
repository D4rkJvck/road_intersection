#[allow(unused)]
pub enum VehicleColor {
    Blue,
    Yellow,
    Orange,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[allow(unused)]
pub struct Vehicle {
    color: VehicleColor,
    speed: u32,
    direction: Direction,
}

#[allow(unused)]
pub enum TrafficLight {
    Red,
    Green,
}

#[allow(unused)]
pub struct Lane {
    vehicles_queue: Vec<Vehicle>,
    traffic_light: TrafficLight,
}

#[allow(unused)]
pub struct Roads {
    vehicules: Vec<Vehicle>,
    traffic_lights: Vec<TrafficLight>,
    lanes: Vec<Lane>,
}
