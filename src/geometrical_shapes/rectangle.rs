// Rectangle
use super::*;

pub struct Rectangle {
    pub point1: Point,
    pub point2: Point,
    color: (u8, u8, u8),
}
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle {
            point1: *p1,
            point2: *p2,
            color: super::random_color(),
        }
    }
}

//Draw Rectangle
impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let min_x = self.point1.x.min(self.point2.x);
        let max_x = self.point1.x.max(self.point2.x);

        let min_y = self.point1.y.min(self.point2.y);
        let max_y = self.point1.y.max(self.point2.y);

        let p1 = Point::new(min_x, min_y);
        let p2 = Point::new(max_x, min_y);
        let p3 = Point::new(max_x, max_y);
        let p4 = Point::new(min_x, max_y);

        let mut line1 = Line::new(&p1, &p2);
        let mut line2 = Line::new(&p2, &p3);
        let mut line3 = Line::new(&p3, &p4);
        let mut line4 = Line::new(&p4, &p1);

        line1.color = self.color;
        line2.color = self.color;
        line3.color = self.color;
        line4.color = self.color;

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
        line4.draw(image);
    }
     fn color(&self) -> Color {
        Color::rgb(self.color.0, self.color.1, self.color.2)
    }
}