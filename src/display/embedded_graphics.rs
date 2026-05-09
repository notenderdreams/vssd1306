use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
};

use super::Display;
use crate::error::DisplayError;

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }
}

impl DrawTarget for Display {
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if point.x < 0 || point.y < 0 {
                continue;
            }

            self.set_pixel(point.x as u32, point.y as u32, color == BinaryColor::On)?;
        }

        Ok(())
    }
}
