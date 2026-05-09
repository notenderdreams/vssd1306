use std::time::{Duration, Instant};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
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

    let renderer = WindowRenderer::new(display, "Animation Example");

    // Start 1 second in the past to trigger immediate first draw
    let mut last_update = Instant::now().checked_sub(Duration::from_secs(1)).unwrap_or_else(Instant::now);
    let mut count = 10;
    
    renderer.run(move |display| {
        let now = Instant::now();
        if now.duration_since(last_update) >= Duration::from_secs(1) {
            last_update = now;
            
            // clear display
            display.clear().unwrap();
            
            let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
            
            if count > 0 {
                let s = format!("Countdown: {}", count);
                Text::new(&s, Point::new(30, 32), text_style)
                    .draw(display)
                    .unwrap();
                count -= 1;
            } else {
                Text::new("Done!", Point::new(45, 32), text_style)
                    .draw(display)
                    .unwrap();
            }
            display.flush().unwrap();
        }
        
        true
    });
}
