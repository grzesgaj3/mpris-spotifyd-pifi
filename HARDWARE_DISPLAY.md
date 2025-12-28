# Hardware SSD1306 Display Implementation

This file contains a complete hardware implementation for the SSD1306 OLED display
when running on Raspberry Pi with actual hardware.

## To enable hardware display:

1. Replace the stub implementation in `src/display.rs` with the code below
2. Ensure all dependencies are available on the Raspberry Pi
3. Connect the SSD1306 display via SPI as documented in README.md

## Complete Hardware Implementation:

```rust
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use rppal::i2c::I2c;
use anyhow::Result;
use crate::mpris_client::TrackInfo;

pub struct Display {
    display: Ssd1306<
        I2CInterface<I2c>,
        DisplaySize128x32,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x32>,
    >,
}

impl Display {
    pub fn new(_spi_device: &str, _dc_pin: u8, _rst_pin: u8) -> Result<Self> {
        // Using I2C instead of SPI for easier setup
        let i2c = I2c::new()?;
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        
        display.init()?;
        display.clear()?;

        log::info!("SSD1306 display initialized");
        Ok(Self { display })
    }

    pub fn render(&mut self, track: &TrackInfo) -> Result<()> {
        self.display.clear();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let title = Self::truncate(&track.title, 21);
        let artist = Self::truncate(&track.artist, 21);

        Text::with_baseline(&title, Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut self.display)?;

        Text::with_baseline(&artist, Point::new(0, 11), text_style, Baseline::Top)
            .draw(&mut self.display)?;

        self.draw_progress_bar(track.progress_percent())?;
        self.display.flush()?;
        Ok(())
    }

    fn draw_progress_bar(&mut self, progress: f32) -> Result<()> {
        let bar_y = 24;
        let bar_height = 6;
        let bar_width = 126;
        
        Rectangle::new(Point::new(1, bar_y), Size::new(bar_width, bar_height))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut self.display)?;

        let filled_width = ((bar_width - 4) as f32 * progress / 100.0) as u32;
        if filled_width > 0 {
            Rectangle::new(Point::new(3, bar_y + 2), Size::new(filled_width, bar_height - 4))
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(&mut self.display)?;
        }

        Ok(())
    }

    fn truncate(s: &str, max_len: usize) -> String {
        if s.chars().count() <= max_len {
            s.to_string()
        } else {
            let mut result: String = s.chars().take(max_len - 3).collect();
            result.push_str("...");
            result
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        self.display.clear();
        self.display.flush()?;
        Ok(())
    }

    pub fn show_message(&mut self, line1: &str, line2: &str) -> Result<()> {
        self.display.clear();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(&Self::truncate(line1, 21), Point::new(0, 0), text_style, Baseline::Top)
            .draw(&mut self.display)?;

        Text::with_baseline(&Self::truncate(line2, 21), Point::new(0, 11), text_style, Baseline::Top)
            .draw(&mut self.display)?;

        self.display.flush()?;
        Ok(())
    }
}
```

## Alternative: Using SPI with newer crates

If you prefer SPI, use updated versions of the crates in Cargo.toml:

```toml
ssd1306 = "0.9"
rppal = "0.22"
embedded-graphics = "0.8"
```

And use rppal's SPI implementation instead of linux-embedded-hal.
