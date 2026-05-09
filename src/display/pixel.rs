use crate::error::DisplayError;

use super::Display;

impl Display {
    pub fn set_pixel(&mut self, x: u32, y: u32, on: bool) -> Result<(), DisplayError> {
        self.ensure_initialized()?;

        let Some((nx, ny)) = self.to_native_coords(x, y) else {
            return Ok(());
        };

        let index = self.byte_index(nx, ny);
        let mask = self.bit_mask(ny);

        if on {
            self.buffer[index] |= mask;
        } else {
            self.buffer[index] &= !mask;
        }

        Ok(())
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Result<bool, DisplayError> {
        self.ensure_initialized()?;

        let Some((nx, ny)) = self.to_native_coords(x, y) else {
            return Ok(false);
        };

        let index = self.byte_index(nx, ny);

        Ok(((self.buffer[index] >> (ny % 8)) & 1) != 0)
    }

    pub(crate) fn byte_index(&self, x: u32, y: u32) -> usize {
        let page = y / 8;

        (page * self.native_width + x) as usize
    }

    pub(crate) fn bit_mask(&self, y: u32) -> u8 {
        1 << (y % 8)
    }
}
