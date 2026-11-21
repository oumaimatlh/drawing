use super::*;

//Line
pub struct Line {
    pub point1: Point,
    pub point2: Point,
    pub color: (u8, u8, u8),
}
impl Line {
    #[allow(dead_code)]
    pub fn new(p1: &Point, p2: &Point) -> Line {
        Line {
            point1: *p1,
            point2: *p2,
            color: Self::random_color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Line {
        Line {
            point1: Point::random(width, height),
            point2: Point::random(width, height),
            color: Self::random_color(),
        }
    }

    fn random_color() -> (u8, u8, u8) {
        super::random_color()
    }
}
impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x1 = self.point1.x;
        let x2 = self.point2.x;
        let y1 = self.point1.y;
        let y2 = self.point2.y;
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        if x1 == x2 {
            let min = y1.min(y2);
            let max = y1.max(y2);

            for y in min..=max {
                image.display(x1, y, self.color());
            }
        }else {
            if dx > dy {
                let min = x1.min(x2);
                let max = x1.max(x2);

                for x in min..=max {
                    let y = y1 + ((y2 - y1) * (x - x1)) / (x2 - x1);
                    image.display(x, y, self.color());
                }
            } else {
                let min = y1.min(y2);
                let max = y1.max(y2);

                for y in min..=max {
                    let x = x1 + ((x2 - x1) * (y - y1)) / (y2 - y1);
                    image.display(x, y, self.color());
                }
            }
        }
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
    fn test_line_random() {
        let line = Line::random(1000, 1000);

        assert!(line.point1.x >= 0 && line.point1.x < 1000);
        assert!(line.point1.y >= 0 && line.point1.y < 1000);

        assert!(line.point2.x >= 0 && line.point2.x < 1000);
        assert!(line.point2.y >= 0 && line.point2.y < 1000);
    }

}