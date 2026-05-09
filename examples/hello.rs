use embedded_graphics::{
    mono_font::{
        ascii::FONT_6X10,
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

use vssd1306::{
    display::Display,
    rotation::DisplayRotation,
    size::DisplaySize,
};

fn main() {
    let mut display = Display::new(
        DisplaySize::DisplaySize128x64,
        DisplayRotation::Rotate0,
    );

    display.init().unwrap();

    let text_style = MonoTextStyle::new(
        &FONT_6X10,
        BinaryColor::On,
    );

    Text::new(
        "Hello World!",
        Point::new(10, 20),
        text_style,
    )
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    println!(
        "{}",
        display.to_ascii().unwrap()
    );
}