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