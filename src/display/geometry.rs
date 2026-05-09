use crate::rotation::DisplayRotation ;

use super::Display;

impl Display {
    pub fn width(&self) -> u32 {
        match self.rotation {
            DisplayRotation::Rotate0
            | DisplayRotation::Rotate180 => {
                self.native_width
            }

            DisplayRotation::Rotate90
            | DisplayRotation::Rotate270 => {
                self.native_height
            }
        }
    }

    pub fn height(&self) -> u32 {
        match self.rotation {
            DisplayRotation::Rotate0
            | DisplayRotation::Rotate180 => {
                self.native_height
            }

            DisplayRotation::Rotate90
            | DisplayRotation::Rotate270 => {
                self.native_width
            }
        }
    }

    pub(crate) fn to_native_coords(
        &self,
        x: u32,
        y: u32,
    ) -> Option<(u32, u32)> {
        let canvas_width = self.width();
        let canvas_height = self.height();

        if x >= canvas_width
            || y >= canvas_height
        {
            return None;
        }

        let coords = match self.rotation {
            DisplayRotation::Rotate0 => {
                (x, y)
            }

            DisplayRotation::Rotate90 => {
                (
                    y,
                    self.native_width - 1 - x,
                )
            }

            DisplayRotation::Rotate180 => {
                (
                    self.native_width - 1 - x,
                    self.native_height - 1 - y,
                )
            }

            DisplayRotation::Rotate270 => {
                (
                    self.native_height - 1 - y,
                    x,
                )
            }
        };

        Some(coords)
    }
}