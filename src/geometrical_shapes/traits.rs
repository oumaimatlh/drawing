use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
