use raster::{Color, Image};

mod circle;
mod line;
mod point;
mod rectangle;
mod traits;
mod triangle;

fn random_color() -> (u8, u8, u8) {
    (
        rand::random_range(0..=255),
        rand::random_range(0..=255),
        rand::random_range(0..=255),
    )
}

pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use rectangle::Rectangle;
pub use traits::{Displayable, Drawable};
pub use triangle::Triangle;
