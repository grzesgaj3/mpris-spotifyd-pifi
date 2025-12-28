mod mpris_client;
mod display;
mod rotary;
mod config;

use anyhow::Result;
use std::time::Duration;
use std::thread;

use mpris_client::{MprisClient, TrackInfo};
use display::Display;
use rotary::{RotaryEncoder, RotaryEvent, ControlMode};
use config::Config;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting MPRIS Spotifyd PiFi application...");

    // Load configuration
    let config = match Config::load("config.toml") {
        Ok(cfg) => {
            log::info!("Loaded configuration from config.toml");
            cfg
        }
        Err(e) => {
            log::warn!("Could not load config.toml ({}), using defaults", e);
            let cfg = Config::default();
            // Try to save default config
            if let Err(e) = cfg.save("config.toml") {
                log::warn!("Could not save default config: {}", e);
            }
            cfg
        }
    };

    // Initialize display
    let mut display = match Display::new(
        &config.display.spi_device,
        config.display.dc_pin,
        config.display.rst_pin,
    ) {
        Ok(d) => {
            log::info!("Display initialized successfully");
            d
        }
        Err(e) => {
            log::error!("Failed to initialize display: {}", e);
            log::warn!("Continuing without display...");
            return Err(e);
        }
    };

    // Show startup message
    display.show_message("MPRIS Spotifyd", "Initializing...")?;
    thread::sleep(Duration::from_secs(2));

    // Initialize rotary encoder
    let mut encoder = match RotaryEncoder::new(
        config.rotary.clk_pin,
        config.rotary.dt_pin,
        config.rotary.sw_pin,
    ) {
        Ok(e) => {
            log::info!("Rotary encoder initialized");
            Some(e)
        }
        Err(e) => {
            log::warn!("Failed to initialize rotary encoder: {}", e);
            log::warn!("Continuing without rotary encoder...");
            None
        }
    };

    // Initialize MPRIS client
    let mut mpris_client = MprisClient::new();
    let mut last_track = TrackInfo::default();
    let mut connection_failed_count = 0;

    loop {
        // Try to connect to MPRIS player if not connected
        if connection_failed_count > 0 {
            match mpris_client.connect() {
                Ok(_) => {
                    log::info!("Successfully connected to player");
                    connection_failed_count = 0;
                    display.show_message("Connected to", "Spotify")?;
                    thread::sleep(Duration::from_secs(1));
                }
                Err(e) => {
                    if connection_failed_count == 1 {
                        log::warn!("Waiting for MPRIS player: {}", e);
                        display.show_message("Waiting for", "Spotify...")?;
                    }
                    connection_failed_count += 1;
                    thread::sleep(Duration::from_millis(config.mpris.reconnect_interval_ms));
                    continue;
                }
            }
        } else if connection_failed_count == 0 {
            // Initial connection attempt
            if let Err(e) = mpris_client.connect() {
                log::warn!("Initial connection failed: {}", e);
                connection_failed_count = 1;
                continue;
            }
        }

        // Get current track info
        match mpris_client.get_track_info() {
            Ok(track) => {
                // Only update display if track info changed significantly
                if track.title != last_track.title 
                    || track.artist != last_track.artist 
                    || (track.position.as_secs() != last_track.position.as_secs()) {
                    
                    if let Err(e) = display.render(&track) {
                        log::error!("Failed to render display: {}", e);
                    }
                    last_track = track;
                }
            }
            Err(e) => {
                log::error!("Failed to get track info: {}", e);
                connection_failed_count = 1;
                continue;
            }
        }

        // Handle rotary encoder events
        if let Some(ref mut enc) = encoder {
            if let Some(event) = enc.poll() {
                let mode = enc.get_mode();
                
                match event {
                    RotaryEvent::ButtonDoubleClick => {
                        let mode_name = match mode {
                            ControlMode::Volume => "Volume Mode",
                            ControlMode::Scroll => "Scroll Mode",
                        };
                        log::info!("Mode switched to: {:?}", mode);
                        if let Err(e) = display.show_message("Mode:", mode_name) {
                            log::error!("Failed to show mode message: {}", e);
                        }
                        thread::sleep(Duration::from_millis(800));
                    }
                    RotaryEvent::ButtonPress => {
                        log::debug!("Button pressed - play/pause");
                        if let Err(e) = mpris_client.play_pause() {
                            log::error!("Failed to toggle play/pause: {}", e);
                        }
                    }
                    RotaryEvent::Clockwise => {
                        match mode {
                            ControlMode::Volume => {
                                if let Ok(current_vol) = mpris_client.get_volume() {
                                    let new_vol = (current_vol + config.rotary.volume_step).min(1.0);
                                    if let Err(e) = mpris_client.set_volume(new_vol) {
                                        log::error!("Failed to set volume: {}", e);
                                    } else {
                                        log::debug!("Volume: {:.0}%", new_vol * 100.0);
                                    }
                                }
                            }
                            ControlMode::Scroll => {
                                log::info!("Next track");
                                if let Err(e) = mpris_client.next() {
                                    log::error!("Failed to skip to next track: {}", e);
                                }
                            }
                        }
                    }
                    RotaryEvent::CounterClockwise => {
                        match mode {
                            ControlMode::Volume => {
                                if let Ok(current_vol) = mpris_client.get_volume() {
                                    let new_vol = (current_vol - config.rotary.volume_step).max(0.0);
                                    if let Err(e) = mpris_client.set_volume(new_vol) {
                                        log::error!("Failed to set volume: {}", e);
                                    } else {
                                        log::debug!("Volume: {:.0}%", new_vol * 100.0);
                                    }
                                }
                            }
                            ControlMode::Scroll => {
                                log::info!("Previous track");
                                if let Err(e) = mpris_client.previous() {
                                    log::error!("Failed to skip to previous track: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Small delay to prevent busy-waiting
        thread::sleep(Duration::from_millis(config.display.update_interval_ms));
    }
}
