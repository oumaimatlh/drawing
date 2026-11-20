use raster::{Color, Image};

//Shapes: 
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


impl Drawable for Point {
    fn draw(&self, image:&mut Image) {
        image.display(self.x, self.y, self.color());
    }
    fn color(&self)-> Color{
        Color::rgb(255, 192, 203)
    }
}