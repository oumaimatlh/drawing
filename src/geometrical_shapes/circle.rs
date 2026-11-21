
use super::*;

//Circle
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}
impl Circle {
    #[allow(dead_code)]
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
