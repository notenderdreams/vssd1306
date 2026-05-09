//! Display Rotation

/// Display Rotation (CLOCK WISE
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DisplayRotation {
    Rotate0,   // user (x,y) → native (x, y)
    Rotate90,  // user (x,y) → native (y,  w−1−x)   canvas: h×w
    Rotate180, // user (x,y) → native (w−1−x, h−1−y)
    Rotate270, // user (x,y) → native (h−1−y, x)     canvas: h×w
}
