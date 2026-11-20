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

//Shape Line: 
pub struct Line {
    pub point1: &Point,
    pub point2: &Point,
}

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
        let x  = self.point1.x;
        let x1 = self.point2.x; 
        let y  = self.point1.y;
        let y1 = self.point2.y; 
        
        let min :char; 
        let max :char; 
        
        if x == x1 {};
        
    


    }
    fn color(&self)-> Color{
        Color::rgb(255, 192, 203)
    }
}