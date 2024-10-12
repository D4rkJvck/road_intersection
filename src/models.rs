use sdl2::pixels::Color;

pub struct Vehicle {
    color: Color,
}

pub enum TrafficLight {
    Red,
    Green
}

pub struct Lane {
    vehicles: Vec<Vehicle>,
    traffic_light: TrafficLight,
}

pub struct Roads {
    vehicules: Vec<Vehicle>,
    traffic_lights: Vec<TrafficLight>,
    lanes: Vec<Lane>,
}
