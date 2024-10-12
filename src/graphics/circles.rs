use sdl2::rect::Point;

/// IDEA: To be refactored...
/// ```
/// use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
/// 
/// const RADIUS: i32 = 4;
/// 
/// pub struct Circle {
///     center: Point,
/// }
/// 
/// impl Circle {
///     pub fn new(center: Point) -> Self {
///         Circle { center }
///     }
/// 
///     pub fn draw(&self, drawer: &mut Canvas<Window>) {
///         let mut x = RADIUS;
///         let mut y = 0;
///         let mut decision_over_2 = 1 - x;
/// 
///         drawer.set_draw_color(Color::RGBA(0, 171, 129, 255));
///         // Draw each octant's points.
///         while y <= x {
///             drawer
///                 .draw_point(Point::new(self.center.x + x, self.center.y + y))
///                 .unwrap();
///             drawer
///                 .draw_point(Point::new(self.center.x - x, self.center.y + y))
///                 .unwrap();
///             drawer
///                 .draw_point(Point::new(self.center.x + x, self.center.y - y))
///                 .unwrap();
///             drawer
///                 .draw_point(Point::new(self.center.x - x, self.center.y - y))
///                 .unwrap();
///             drawer
///                 .draw_point(Point::new(self.center.x + y, self.center.y + x))
///                 .unwrap();
///             drawer
///                 .draw_point(Point::new(self.center.x - y, self.center.y + x))
///                 .unwrap();
///             drawer
///                 .draw_point(Point::new(self.center.x + y, self.center.y - x))
///                 .unwrap();
///             drawer
///                 .draw_point(Point::new(self.center.x - y, self.center.y - x))
///                 .unwrap();
///             y += 1; // Move vertically downwards
///             if decision_over_2 <= 0 {
///                 decision_over_2 += 2 * y + 1; // Move horizontally if inside or on the circle
///             } else {
///                 x -= 1; // Move horizontally left if outside
///                 decision_over_2 -= 2 * (y - x) + 1; // Ajust for next points
///             }
///         }
///     }
/// }
/// ```
pub struct Circle {
    center: Point
}
