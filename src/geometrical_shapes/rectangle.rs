use super::*;

// Rectangle
pub struct Rectangle {
    pub point1: Point,
    pub point2: Point,
}
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle {
            point1: *p1,
            point2: *p2,
        }
    }
}
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

        let edges = [
            (&p1, &p2),
            (&p2, &p3),
            (&p3, &p4),
            (&p4, &p1),
        ];

        for (a, b) in edges {
            let x1 = a.x;
            let x2 = b.x;
            let y1 = a.y;
            let y2 = b.y;

            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    image.display(x1, y, self.color());
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    let y = y1 + ((y2 - y1) * (x - x1)) / (x2 - x1);
                    image.display(x, y, self.color());
                }
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(239, 255, 13)
    }
}