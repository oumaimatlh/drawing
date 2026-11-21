
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

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

//Line
pub struct Line {
    pub point1: Point,
    pub point2: Point,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Line {
        Line {
            point1: *p1,
            point2: *p2,
        }
    }

    pub fn random(width: i32, height: i32) -> Line {
        Line {
            point1: Point::random(width, height),
            point2: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x1 = self.point1.x;
        let x2 = self.point2.x;
        let y1 = self.point1.y;
        let y2 = self.point2.y;

        if x1 == x2 {
            let min = y1.min(y2);
            let max = y1.max(y2);

            for y in min..=max {
                image.display(x1, y, self.color());
            }
        } else {
            let min = x1.min(x2);
            let max = x1.max(x2);

            for x in min..=max {
                let y = y1 + ((y2 - y1) * (x - x1)) / (x2 - x1);
                image.display(x, y, self.color());
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(247, 246, 207)
    }
}

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

//Circle
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(c: &Point, r: i32) -> Circle {
        Circle {
            center: *c,
            radius: r,
        }
    }

    pub fn random(width: i32, height: i32) -> Circle {
        let center = Point::random(width, height);

        let max_radius = (width.min(height) / 4).max(1);
        let radius = rand::random_range(1..=max_radius);

        Circle {
            center,
            radius,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let xc = self.center.x;
        let yc = self.center.y;
        let r = self.radius;

        for x in (xc - r)..=(xc + r) {
            for y in (yc - r)..=(yc + r) {
                let dx = x - xc;
                let dy = y - yc;

                let distance = ((dx * dx + dy * dy) as f64).sqrt();

                if (distance - r as f64).abs() <= 0.5 {
                    image.display(x, y, self.color());
                }
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 196, 221)

    }
}


//Units-test :
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_random() {
        let point = Point::random(1000, 1000);

        assert!(point.x >= 0 && point.x < 1000);
        assert!(point.y >= 0 && point.y < 1000);
    }

    #[test]
    fn test_line_random() {
        let line = Line::random(1000, 1000);

        assert!(line.point1.x >= 0 && line.point1.x < 1000);
        assert!(line.point1.y >= 0 && line.point1.y < 1000);

        assert!(line.point2.x >= 0 && line.point2.x < 1000);
        assert!(line.point2.y >= 0 && line.point2.y < 1000);
    }

    #[test]
    fn test_circle_random() {
        let circle = Circle::random(1000, 1000);

        assert!(circle.center.x >= 0 && circle.center.x < 1000);
        assert!(circle.center.y >= 0 && circle.center.y < 1000);
    }
}