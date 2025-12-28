use anyhow::Result;
use rppal::gpio::{Gpio, InputPin, Level};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotaryEvent {
    Clockwise,
    CounterClockwise,
    ButtonPress,
    ButtonDoubleClick,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlMode {
    Volume,
    Scroll,
}

pub struct RotaryEncoder {
    clk_pin: InputPin,
    dt_pin: InputPin,
    sw_pin: InputPin,
    last_clk: Level,
    last_button_press: Option<Instant>,
    mode: Arc<Mutex<ControlMode>>,
}

impl RotaryEncoder {
    pub fn new(clk: u8, dt: u8, sw: u8) -> Result<Self> {
        let gpio = Gpio::new()?;
        
        let mut clk_pin = gpio.get(clk)?.into_input_pullup();
        let dt_pin = gpio.get(dt)?.into_input_pullup();
        let mut sw_pin = gpio.get(sw)?.into_input_pullup();

        // Set up interrupts for better responsiveness
        clk_pin.set_interrupt(rppal::gpio::Trigger::Both)?;
        sw_pin.set_interrupt(rppal::gpio::Trigger::FallingEdge)?;

        let last_clk = clk_pin.read();

        Ok(Self {
            clk_pin,
            dt_pin,
            sw_pin,
            last_clk,
            last_button_press: None,
            mode: Arc::new(Mutex::new(ControlMode::Volume)),
        })
    }

    pub fn poll(&mut self) -> Option<RotaryEvent> {
        // Check for button press
        if let Ok(Some(_)) = self.sw_pin.poll_interrupt(false, Some(Duration::from_millis(1))) {
            let now = Instant::now();
            
            if let Some(last_press) = self.last_button_press {
                // Check if this is a double-click (within 500ms)
                if now.duration_since(last_press) < Duration::from_millis(500) {
                    self.last_button_press = None;
                    
                    // Toggle mode on double-click
                    let mut mode = self.mode.lock().unwrap();
                    *mode = match *mode {
                        ControlMode::Volume => ControlMode::Scroll,
                        ControlMode::Scroll => ControlMode::Volume,
                    };
                    log::info!("Switched to {:?} mode", *mode);
                    
                    return Some(RotaryEvent::ButtonDoubleClick);
                }
            }
            
            self.last_button_press = Some(now);
            return Some(RotaryEvent::ButtonPress);
        }

        // Check for rotation
        let clk = self.clk_pin.read();
        
        if clk != self.last_clk && clk == Level::Low {
            let dt = self.dt_pin.read();
            self.last_clk = clk;
            
            if dt == Level::High {
                return Some(RotaryEvent::Clockwise);
            } else {
                return Some(RotaryEvent::CounterClockwise);
            }
        }
        
        self.last_clk = clk;
        None
    }

    pub fn get_mode(&self) -> ControlMode {
        *self.mode.lock().unwrap()
    }
}
