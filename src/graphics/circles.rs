use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

const RADIUS: i32 = 20;

pub struct Circle {
    center: Point,
    color: Color,
}

impl Circle {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        let center = Point::new(x, y);
        Self { center, color }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let mut x = RADIUS;
        let mut y = 0;
        let mut decision_over_2 = 1 - x;

        canvas.set_draw_color(self.color);

        // Draw each octant's points.
        while y <= x {
            canvas.draw_point(Point::new(self.center.x + x, self.center.y + y))?;
            canvas.draw_point(Point::new(self.center.x - x, self.center.y + y))?;
            canvas.draw_point(Point::new(self.center.x + x, self.center.y - y))?;
            canvas.draw_point(Point::new(self.center.x - x, self.center.y - y))?;
            canvas.draw_point(Point::new(self.center.x + y, self.center.y + x))?;
            canvas.draw_point(Point::new(self.center.x - y, self.center.y + x))?;
            canvas.draw_point(Point::new(self.center.x + y, self.center.y - x))?;
            canvas.draw_point(Point::new(self.center.x - y, self.center.y - x))?;

            y += 1; // Move vertically downwards

            if decision_over_2 <= 0 {
                decision_over_2 += 2 * y + 1; // Move horizontally if inside or on the circle
            } else {
                x -= 1; // Move horizontally left if outside

                decision_over_2 -= 2 * (y - x) + 1; // Ajust for next points
            }
        }

        Ok(())
    }
}
