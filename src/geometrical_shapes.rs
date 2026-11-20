use raster::{Color, Image};
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

