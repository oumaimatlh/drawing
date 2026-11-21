
use super::*;

// Triangle 

pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}
impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Triangle {
        Triangle {
            point1: *p1,
            point2: *p2,
            point3: *p3,
        }
    }
}
impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let points = [
            (&self.point1, &self.point2),
            (&self.point2, &self.point3),
            (&self.point3, &self.point1),
        ];

        for (p1, p2) in points {
            let x1 = p1.x;
            let x2 = p2.x;
            let y1 = p1.y;
            let y2 = p2.y;

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
        Color::rgb(128, 196, 255)
    }
}