#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DisplaySize {
    DisplaySize128x64, // 128 × 8  = 1024 bytes
    DisplaySize128x32, // 128 × 4  =  512 bytes
    DisplaySize96x16,  // 96  × 2  =  192 bytes
    DisplaySize72x40,  // 72  × 5  =  360 bytes
    DisplaySize64x48,  // 64  × 6  =  384 bytes
}

impl DisplaySize {
    pub fn dimensions(self) -> (u32, u32) {
        match self {
            DisplaySize::DisplaySize128x64 => (128, 64),
            DisplaySize::DisplaySize128x32 => (128, 32),
            DisplaySize::DisplaySize96x16 => (96, 16),
            DisplaySize::DisplaySize72x40 => (72, 40),
            DisplaySize::DisplaySize64x48 => (64, 48),
        }
    }
}
