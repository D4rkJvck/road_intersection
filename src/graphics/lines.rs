use sdl2::rect::Point;

pub struct Line(Point, Point);

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self(Point::new(x1, y1), Point::new(x2, y2))
    }

    pub fn get_start(&self) -> Point {
        self.0
    }

    pub fn get_end(&self) -> Point {
        self.1
    }
}

pub struct Roads {
    vertical: Vec<Line>,
    horizontal: Vec<Line>,
}

impl Roads {
    pub fn new(width: u32, height: u32) -> Self {
        let mid_height = height as i32 / 2;
        let mid_width = width as i32 / 2;

        let top_horizontal = Line::new(0, mid_height - 50, width as i32, mid_height - 50);
        let mid_horizontal = Line::new(0, mid_height, width as i32, mid_height);
        let bottom_horizontal = Line::new(0, mid_height + 50, width as i32, mid_height + 50);

        let right_vertical = Line::new(mid_width - 50, 0, mid_width - 50, height as i32);
        let mid_vertical = Line::new(mid_width, 0, mid_width, height as i32);
        let left_vertical = Line::new(mid_width + 50, 0, mid_width + 50, height as i32);

        let horizontal = vec![top_horizontal, mid_horizontal, bottom_horizontal];
        let vertical = vec![right_vertical, mid_vertical, left_vertical];

        Self {
            vertical,
            horizontal,
        }
    }

    pub fn get_vertical(&self) -> &Vec<Line> {
        &self.vertical
    }

    pub fn get_horizontal(&self) -> &Vec<Line> {
        &self.horizontal
    }
}
