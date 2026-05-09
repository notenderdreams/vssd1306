use crate::{
    brightness::Brightness, error::DisplayError, rotation::DisplayRotation, size::DisplaySize,
};

pub struct Display {
    #[allow(dead_code)]
    pub(crate) size: DisplaySize,

    pub(crate) native_width: u32,
    pub(crate) native_height: u32,

    pub(crate) rotation: DisplayRotation,
    pub(crate) brightness: Brightness,

    pub(crate) initialized: bool,
    pub(crate) inverted: bool,

    pub(crate) buffer: Vec<u8>,
    pub(crate) visible: Vec<u8>,
}

impl Display {
    pub fn new(size: DisplaySize, rotation: DisplayRotation) -> Self {
        let (w, h) = size.dimensions();

        Self {
            size,

            native_width: w,
            native_height: h,

            rotation,
            brightness: Brightness::Normal,

            initialized: false,
            inverted: false,

            buffer: Vec::new(),
            visible: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), DisplayError> {
        let bytes = (self.native_width * self.native_height / 8) as usize;

        self.buffer = vec![0; bytes];
        self.visible = vec![0; bytes];

        self.initialized = true;

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.ensure_initialized()?;

        self.visible.copy_from_slice(&self.buffer);

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), DisplayError> {
        self.ensure_initialized()?;

        self.buffer.fill(0);

        Ok(())
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn visible_buffer(&self) -> &[u8] {
        &self.visible
    }

    pub fn set_rotation(&mut self, rotation: DisplayRotation) -> Result<(), DisplayError> {
        self.ensure_initialized()?;

        self.rotation = rotation;

        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: Brightness) -> Result<(), DisplayError> {
        self.ensure_initialized()?;

        self.brightness = brightness;

        Ok(())
    }

    pub fn set_inverted(&mut self, inverted: bool) -> Result<(), DisplayError> {
        self.ensure_initialized()?;

        self.inverted = inverted;

        Ok(())
    }

    pub fn visible_pixel(&self, x: u32, y: u32) -> Result<bool, DisplayError> {
        self.ensure_initialized()?;

        let Some((nx, ny)) = self.to_native_coords(x, y) else {
            return Ok(false);
        };

        let index = self.byte_index(nx, ny);

        let mut value = ((self.visible[index] >> (ny % 8)) & 1) != 0;

        if self.inverted {
            value = !value;
        }

        Ok(value)
    }

    pub fn to_ascii(&self) -> Result<String, DisplayError> {
        self.ensure_initialized()?;

        let mut output = String::new();

        let rows = self.height().div_ceil(2);

        for row in 0..rows {
            for x in 0..self.width() {
                let top = self.visible_pixel(x, row * 2)?;

                let bottom = self.visible_pixel(x, row * 2 + 1)?;

                let ch = match (top, bottom) {
                    (false, false) => ' ',
                    (true, false) => '▀',
                    (false, true) => '▄',
                    (true, true) => '█',
                };

                output.push(ch);
            }

            output.push('\n');
        }

        Ok(output)
    }

    pub fn to_pgm(&self) -> Result<Vec<u8>, DisplayError> {
        self.ensure_initialized()?;

        let width = self.width();
        let height = self.height();

        let mut out = Vec::new();

        let header = format!("P5\n{} {}\n255\n", width, height);

        out.extend_from_slice(header.as_bytes());

        for y in 0..height {
            for x in 0..width {
                let pixel = self.visible_pixel(x, y)?;

                out.push(if pixel { 0xFF } else { 0x00 });
            }
        }

        Ok(out)
    }

    pub(crate) fn ensure_initialized(&self) -> Result<(), DisplayError> {
        if !self.initialized {
            Err(DisplayError::NotInitialized)
        } else {
            Ok(())
        }
    }
}
