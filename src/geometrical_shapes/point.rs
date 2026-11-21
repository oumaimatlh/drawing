//POint
use super::*;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    color: (u8, u8, u8),
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            color: super::random_color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Point {
        Point {
            x: rand::random_range(0..width),
            y: rand::random_range(0..height),
            color: super::random_color(),
        }
    }
}
impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(self.color.0, self.color.1, self.color.2)
    }
}

//Units-test :
#[cfg(test)]
mod tests {
    use super::super::*;
    #[test]
    fn test_point_random() {
        let point = Point::random(1000, 1000);

        assert!(point.x >= 0 && point.x < 1000);
        assert!(point.y >= 0 && point.y < 1000);
    }
}