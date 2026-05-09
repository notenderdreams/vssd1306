use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

use vssd1306::{
    display::Display, rotation::DisplayRotation, size::DisplaySize, window::WindowRenderer,
};

fn main() {
    let mut display = Display::new(DisplaySize::DisplaySize128x64, DisplayRotation::Rotate0);

    display.init().unwrap();

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    Text::new("SSD1306 Virtual!", Point::new(10, 20), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw some pixels
    for x in 0..128 {
        display.set_pixel(x, 32, true).unwrap();
    }

    display.flush().unwrap();

    let renderer = WindowRenderer::new(display, "Virtual SSD1306");
    renderer.run();
}
