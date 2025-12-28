use anyhow::Result;
use crate::mpris_client::TrackInfo;

// Simple display abstraction that can work with or without actual hardware
pub struct Display {
    #[allow(dead_code)]
    enabled: bool,
}

impl Display {
    pub fn new(_spi_device: &str, _dc_pin: u8, _rst_pin: u8) -> Result<Self> {
        // In a real implementation on Raspberry Pi, this would initialize the SSD1306
        // For now, we'll create a stub that allows the application to compile and run
        // without the actual hardware
        
        log::warn!("Display initialization - using stub implementation");
        log::warn!("For actual hardware, ensure SSD1306 display is connected via SPI");
        
        Ok(Self { enabled: false })
    }

    pub fn render(&mut self, track: &TrackInfo) -> Result<()> {
        // Log the track info instead of displaying it
        log::info!("♪ {} - {} [{:.0}%]", 
            Self::truncate(&track.artist, 21),
            Self::truncate(&track.title, 21),
            track.progress_percent()
        );
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

    #[allow(dead_code)]
    pub fn clear(&mut self) -> Result<()> {
        log::debug!("Display cleared");
        Ok(())
    }

    pub fn show_message(&mut self, line1: &str, line2: &str) -> Result<()> {
        log::info!("MSG: {} | {}", line1, line2);
        Ok(())
    }
}

// For Raspberry Pi with actual hardware, use this implementation instead:
#[cfg(all(target_arch = "arm", target_os = "linux"))]
mod hardware_display {
    use super::*;
    use ssd1306::{
        mode::BufferedGraphicsMode,
        prelude::*,
        Ssd1306,
    };
    use linux_embedded_hal::SpidevDevice;
    use linux_embedded_hal::sysfs_gpio::Pin;

    pub struct HardwareDisplay {
        // This would contain the actual SSD1306 display driver
        // display: Ssd1306<...>,
    }

    impl HardwareDisplay {
        pub fn new(spi_device: &str, dc_pin: u8, rst_pin: u8) -> Result<Self> {
            use linux_embedded_hal::spidev::{SpidevOptions, SpiModeFlags};
            use linux_embedded_hal::sysfs_gpio::Direction;

            // Open SPI device
            let mut spi = SpidevDevice::open(spi_device)?;
            let options = SpidevOptions::new()
                .bits_per_word(8)
                .max_speed_hz(8_000_000)
                .mode(SpiModeFlags::SPI_MODE_0)
                .build();
            spi.configure(&options)?;

            // Setup DC and RST pins
            let dc = Pin::new(dc_pin as u64);
            dc.export()?;
            dc.set_direction(Direction::Out)?;

            let mut rst = Pin::new(rst_pin as u64);
            rst.export()?;
            rst.set_direction(Direction::Out)?;

            // Reset display
            rst.set_value(0)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
            rst.set_value(1)?;
            std::thread::sleep(std::time::Duration::from_millis(10));

            log::info!("Hardware display initialized");
            
            // Note: Actual SSD1306 initialization would go here
            // This is a placeholder for the real hardware implementation
            
            Ok(Self {})
        }

        pub fn render(&mut self, track: &TrackInfo) -> Result<()> {
            // Actual rendering to SSD1306 would go here
            log::info!("♪ {} - {} [{:.0}%]", 
                track.artist,
                track.title,
                track.progress_percent()
            );
            Ok(())
        }
    }
}
