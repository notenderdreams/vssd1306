use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::Text,
};

use vssd1306::{
    display::Display, rotation::DisplayRotation, size::DisplaySize,
};

#[cfg(feature = "window")]
use vssd1306::window::WindowRenderer;

fn main() {
    // Create a new display instance (128x64 pixels)
    let mut display = Display::new(DisplaySize::DisplaySize128x64, DisplayRotation::Rotate0);

    // Initialize the display
    display.init().unwrap();

    // Define a stroke style (1 pixel wide, color "On")
    let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    // 1. Draw a rectangle in the center
    // Centering a 60x30 rectangle on a 128x64 display
    let rect = Rectangle::new(Point::new(34, 12), Size::new(60, 30));
    rect.into_styled(style).draw(&mut display).unwrap();

    // 2. Connect the corners with diagonal lines
    let top_left = rect.top_left;
    let bottom_right = rect.bottom_right().unwrap();
    let top_right = Point::new(bottom_right.x, top_left.y);
    let bottom_left = Point::new(top_left.x, bottom_right.y);

    // Diagonal 1: Top-left to Bottom-right
    Line::new(top_left, bottom_right)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // Diagonal 2: Top-right to Bottom-left
    Line::new(top_right, bottom_left)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // 3. Draw "Hello World" at the bottom
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello World!", Point::new(30, 55), text_style)
        .draw(&mut display)
        .unwrap();

    // Flush the changes to the display buffer
    display.flush().unwrap();

    // Show the result in a window if the "window" feature is enabled
    #[cfg(feature = "window")]
    {
        println!("Opening virtual window...");
        let renderer = WindowRenderer::new(display, "Rectangle & Diagonals");
        renderer.run(|_| true);
    }

    // Fallback to ASCII output in the console if the "window" feature is not enabled
    #[cfg(not(feature = "window"))]
    {
        println!("Window feature not enabled. Printing ASCII representation:");
        println!("{}", display.to_ascii().unwrap());
    }
}
