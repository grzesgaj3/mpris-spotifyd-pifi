# Wiring Guide for Raspberry Pi Zero WH + PiFi DAC+ + SSD1306 + Rotary Encoder

## Component List
- Raspberry Pi Zero WH
- PiFi DAC+ (or similar I2S DAC)
- SSD1306 OLED Display 128x32 (SPI interface)
- Rotary Encoder with push button (KY-040 or similar)
- Jumper wires

## Raspberry Pi Zero WH GPIO Pinout

```
     3V3  (1) (2)  5V    
   GPIO2  (3) (4)  5V    
   GPIO3  (5) (6)  GND   
   GPIO4  (7) (8)  GPIO14
     GND  (9) (10) GPIO15
  GPIO17 (11) (12) GPIO18
  GPIO27 (13) (14) GND   
  GPIO22 (15) (16) GPIO23
     3V3 (17) (18) GPIO24
  GPIO10 (19) (20) GND   
   GPIO9 (21) (22) GPIO25
  GPIO11 (23) (24) GPIO8 
     GND (25) (26) GPIO7 
   GPIO0 (27) (28) GPIO1 
   GPIO5 (29) (30) GND   
   GPIO6 (31) (32) GPIO12
  GPIO13 (33) (34) GND   
  GPIO19 (35) (36) GPIO16
  GPIO26 (37) (38) GPIO20
     GND (39) (40) GPIO21
```

## SSD1306 OLED Display Wiring (SPI)

| SSD1306 Pin | RPi Zero Pin | GPIO | Description |
|-------------|--------------|------|-------------|
| VCC         | Pin 1        | 3.3V | Power       |
| GND         | Pin 6        | GND  | Ground      |
| SCL (SCLK)  | Pin 23       | GPIO11 | SPI Clock |
| SDA (MOSI)  | Pin 19       | GPIO10 | SPI Data  |
| DC          | Pin 18       | GPIO24 | Data/Command |
| RST (RES)   | Pin 22       | GPIO25 | Reset     |
| CS          | Pin 24       | GPIO8  | Chip Select |

## Rotary Encoder Wiring

| Encoder Pin | RPi Zero Pin | GPIO | Description |
|-------------|--------------|------|-------------|
| +           | Pin 1        | 3.3V | Power       |
| GND         | Pin 9        | GND  | Ground      |
| CLK         | Pin 11       | GPIO17 | Clock     |
| DT          | Pin 13       | GPIO27 | Data      |
| SW          | Pin 15       | GPIO22 | Switch    |

## PiFi DAC+ Notes

The PiFi DAC+ uses the following pins (these are reserved and should not be used for other purposes):
- GPIO2, GPIO3 (I2C for DAC control)
- GPIO18, GPIO19, GPIO20, GPIO21 (I2S for audio)

These pins are **NOT available** for the display or rotary encoder.

## Configuration Changes

If you need to change pin assignments, edit `config.toml`:

```toml
[display]
dc_pin = 24    # Change this to use different GPIO for DC
rst_pin = 25   # Change this to use different GPIO for RST

[rotary]
clk_pin = 17   # Change this to use different GPIO for CLK
dt_pin = 27    # Change this to use different GPIO for DT
sw_pin = 22    # Change this to use different GPIO for SW
```

## Important Notes

1. **Always use 3.3V**, not 5V, for GPIO devices on Raspberry Pi
2. The SPI interface must be enabled via `raspi-config`
3. Double-check all connections before powering on
4. Use current-limiting resistors if needed (usually not required for these components)
5. The PiFi DAC+ reserves several GPIO pins for audio - avoid using these

## Enabling SPI

```bash
sudo raspi-config
# Navigate to: Interface Options → SPI → Yes
sudo reboot
```

## Testing Connections

```bash
# Check SPI devices
ls -l /dev/spi*

# Check GPIO export
ls -l /sys/class/gpio/

# Test display (after building the app)
sudo ./target/release/mpris-spotifyd-pifi
```

## Troubleshooting

### Display not working
- Verify SPI is enabled: `ls /dev/spi*`
- Check wiring, especially DC and RST pins
- Verify 3.3V power supply
- Check `/var/log/syslog` for errors

### Rotary encoder not responsive
- Verify GPIO pin numbers in config.toml
- Check pull-up resistors (may be built into encoder)
- Test with `gpio readall` command

### PiFi DAC+ conflicts
- Ensure no pin conflicts with I2C (GPIO2, GPIO3)
- Ensure no pin conflicts with I2S (GPIO18-21)
- Check `dtoverlay` configuration in `/boot/config.txt`
