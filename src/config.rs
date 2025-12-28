use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub rotary: RotaryConfig,
    pub mpris: MprisConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayConfig {
    #[serde(default = "default_spi_device")]
    pub spi_device: String,
    #[serde(default = "default_dc_pin")]
    pub dc_pin: u8,
    #[serde(default = "default_rst_pin")]
    pub rst_pin: u8,
    #[serde(default = "default_update_interval")]
    pub update_interval_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RotaryConfig {
    #[serde(default = "default_clk_pin")]
    pub clk_pin: u8,
    #[serde(default = "default_dt_pin")]
    pub dt_pin: u8,
    #[serde(default = "default_sw_pin")]
    pub sw_pin: u8,
    #[serde(default = "default_volume_step")]
    pub volume_step: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MprisConfig {
    #[serde(default = "default_reconnect_interval")]
    pub reconnect_interval_ms: u64,
}

fn default_spi_device() -> String {
    "/dev/spidev0.0".to_string()
}

fn default_dc_pin() -> u8 {
    24
}

fn default_rst_pin() -> u8 {
    25
}

fn default_update_interval() -> u64 {
    500
}

fn default_clk_pin() -> u8 {
    17
}

fn default_dt_pin() -> u8 {
    27
}

fn default_sw_pin() -> u8 {
    22
}

fn default_volume_step() -> f64 {
    0.05
}

fn default_reconnect_interval() -> u64 {
    5000
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                spi_device: default_spi_device(),
                dc_pin: default_dc_pin(),
                rst_pin: default_rst_pin(),
                update_interval_ms: default_update_interval(),
            },
            rotary: RotaryConfig {
                clk_pin: default_clk_pin(),
                dt_pin: default_dt_pin(),
                sw_pin: default_sw_pin(),
                volume_step: default_volume_step(),
            },
            mpris: MprisConfig {
                reconnect_interval_ms: default_reconnect_interval(),
            },
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
}
