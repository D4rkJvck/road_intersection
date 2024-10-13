pub enum VehicleColor {
    Blue,
    Yellow,
    Orange
}

pub struct Vehicle {
    color: VehicleColor,
    speed: u32,
}

pub enum TrafficLight {
    Red,
    Green
}

pub struct Lane {
    vehicles_queue: Vec<Vehicle>,
    traffic_light: TrafficLight,
}

pub struct Roads {
    vehicules: Vec<Vehicle>,
    traffic_lights: Vec<TrafficLight>,
    lanes: Vec<Lane>,
}
