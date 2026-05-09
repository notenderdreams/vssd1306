#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Brightness {
    Dimmest,    // 0x00
    Dim,        // 0x40
    Normal,     // 0x8F (Default)
    Bright,     // 0xCF
    Brightest,  // 0xFF
    Custom(u8), // full control
}

impl Brightness {
    pub fn value(self) -> u8 {
        match self {
            Brightness::Dimmest => 0x00,
            Brightness::Dim => 0x40,
            Brightness::Normal => 0x8F,
            Brightness::Bright => 0xCF,
            Brightness::Brightest => 0xFF,
            Brightness::Custom(v) => v,
        }
    }
}
