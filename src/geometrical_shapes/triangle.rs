// Triangle
use super::*;

pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
    color: (u8, u8, u8),
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Triangle {
        Triangle {
            point1: *p1,
            point2: *p2,
            point3: *p3,
            color: super::random_color(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let mut line1 = Line::new(&self.point1, &self.point2);
        let mut line2 = Line::new(&self.point2, &self.point3);
        let mut line3 = Line::new(&self.point3, &self.point1);

        line1.color = self.color;
        line2.color = self.color;
        line3.color = self.color;

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(self.color.0, self.color.1, self.color.2)
    }
}