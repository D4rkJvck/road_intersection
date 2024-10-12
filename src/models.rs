pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

pub enum TrafficLight {
    Red,
    Green,
}

pub struct Vehicle {
    color: Color,
}

pub struct Roads {
    vehicules: Vec<Vehicle>,
    traffic_lights: Vec<TrafficLight>,
}
