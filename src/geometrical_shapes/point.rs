use super::*;

//POint
#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Point {
        Point {
            x: rand::random_range(0..width),
            y: rand::random_range(0..height),
        }
    }
}
impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(114, 253, 75)
    }
}