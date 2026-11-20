use raster::{Color, Image};
use rand::Rng;

//Traits: 
pub trait Drawable {
    fn draw(&self, image:&mut Image);
    fn color(&self) -> Color; 
}


pub trait Displayable {
    fn display(&mut self, x:i32, y:i32, color:Color);
}


impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

//--------------------//

//Shape Point: 
pub struct Point {
    pub x: i32, //coordonnée horizontal 
    pub y: i32, //coordonnée vertical 
}
impl Point {
    pub fn new(x:i32, y:i32) -> Point {
        Point {
            x,
            y
        }
    }
    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        Point {
            x,
            y,
        }
    }
}
//Draw Point : 
impl Drawable for Point {
    fn draw(&self, image:&mut Image) {
        image.display(self.x, self.y, self.color());
    }
    fn color(&self)-> Color{
        Color::rgb(255, 192, 203)
    }
}

//--------------------//

//Shape Line: 
pub struct Line {
    pub point1: &Point,
    pub point2: &Point,
}
//Insrance Line
impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Line {
        Line {
            point1: p1,
            point2: p2,
        }
    }
}
//Draw Line:
impl Drawable for Line {
    fn draw(&self, image:&mut Image) {
        let x1  = self.point1.x;
        let x2  = self.point2.x; 
        let y1  = self.point1.y;
        let y2  = self.point2.y; 
        
        let mut min :i32 = x1; 
        let mut max :i32 = x2; 

        if x1 == x2 {   
            if y1 > y2 {
                min = y2; 
                max = y1;
            }else {
                min = y1; 
                max = y2; 
            }

            for y in min..=max {
                image.display(x1, y, self.color());
            }
        }else {
               //
            if x1 > x2 {
                min = x2; 
                max = x1;
            }

            for x in min..=max {
                let y = y1 + ((y2-y1)*(x-x1)) / (x2 - x1);
                image.display(x, y, self.color());
            }
        }
     
    }
    fn color(&self)-> Color{
        Color::rgb(247, 246, 207)
    }
}



//---------------//

//Shape Triangle
pub struct Triangle {
    pub point1: &Point,
    pub point2: &Point,
    pub point3: &Point,
}
//Inst Triangle
impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Triangle {
        Triangle {
            point1: p1, //&Point
            point2: p2, 
            point3: p3,
        }
    }
}
//Draw triangle
impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let line1 = Line::new(self.point1, self.point2);
        let line2 = Line::new(self.point2, self.point3);
        let line3 = Line::new(self.point3, self.point1);
        
        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }
    fn color(&self) -> Color {
        Color::rgb(0, 66, 123)
    }
}

//-----------------------//

//Shape Rectangle
pub struct Rectangle {
    pub point1: &Point,
    pub point2: &Point,
}
//Inst Rectangle
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle {
            point1: p1, 
            point2: p2,
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

        Line::new(&p1, &p2).draw(image);
        Line::new(&p2, &p3).draw(image);
        Line::new(&p3, &p4).draw(image);
        Line::new(&p4, &p1).draw(image);
    }
    fn color(&self) -> Color {
        Color::rgb(123, 0, 0)
    }
}

//------------//


//Shape Cercle
pub struct Cercle {
    pub center: &Point,
    pub radius: i32,
}
//Inst Cercle
impl Cercle {
    pub fn new(c: &Point, r: i32) -> Cercle {
        Cercle {
            center: c, 
            point2: r,
        }
    }
}
//Draw Cercle
impl Drawable for Cercle {
    fn draw(&self, image: &mut Image) {
    let xc = self.center.x;
    let yc = self.center.y;
    let r = self.radius;

    let min_x = xc - r;
    let max_x = xc + r;
    let min_y = yc - r;
    let max_y = yc + r;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let dx = x - xc;
                let dy = y - yc;

                if dx * dx + dy * dy == r * r {
                    image.display(x, y, self.color());
                }
            }
        }
    }
    fn color(&self) -> Color {
        Color::rgb(243, 66, 110)
    }
}