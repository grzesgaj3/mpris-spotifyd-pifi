# Quick Start Guide

Quick reference for getting up and running with MPRIS Spotifyd PiFi.

## Prerequisites Checklist

- [ ] Raspberry Pi Zero WH with Raspberry Pi OS installed
- [ ] PiFi DAC+ or compatible audio HAT
- [ ] SSD1306 OLED display (128x32, SPI)
- [ ] Rotary encoder with push button
- [ ] Internet connection
- [ ] Spotify Premium account

## 5-Minute Setup (Experienced Users)

```bash
# 1. System prep
sudo apt update && sudo apt install -y git curl build-essential pkg-config libdbus-1-dev libasound2-dev
sudo raspi-config # Enable SPI

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Install Spotifyd
cd ~
wget https://github.com/Spotifyd/spotifyd/releases/latest/download/spotifyd-linux-armhf-default.tar.gz
tar xzf spotifyd-linux-armhf-default.tar.gz
sudo mv spotifyd /usr/local/bin/

# 4. Configure Spotifyd
mkdir -p ~/.config/spotifyd
cat > ~/.config/spotifyd/spotifyd.conf << EOF
[global]
username = "YOUR_SPOTIFY_USERNAME"
password = "YOUR_SPOTIFY_PASSWORD"
backend = "alsa"
device_name = "PiFi Speaker"
bitrate = 320
EOF

# 5. Clone and build this project
cd ~
git clone https://github.com/grzesgaj3/mpris-spotifyd-pifi.git
cd mpris-spotifyd-pifi
cargo build --release

# 6. Install and run
sudo cp target/release/mpris-spotifyd-pifi /usr/local/bin/
sudo cp mpris-spotifyd-pifi.service /etc/systemd/system/
cp config.toml ~/.config/mpris-spotifyd-pifi.toml

sudo systemctl daemon-reload
sudo systemctl enable --now spotifyd mpris-spotifyd-pifi
```

## Wiring Quick Reference

### SSD1306 Display (SPI)
```
VCC  → Pin 1  (3.3V)
GND  → Pin 6  (GND)
SCL  → Pin 23 (GPIO11)
SDA  → Pin 19 (GPIO10)
DC   → Pin 18 (GPIO24)
RST  → Pin 22 (GPIO25)
CS   → Pin 24 (GPIO8)
```

### Rotary Encoder
```
+    → Pin 1  (3.3V)
GND  → Pin 9  (GND)
CLK  → Pin 11 (GPIO17)
DT   → Pin 13 (GPIO27)
SW   → Pin 15 (GPIO22)
```

## Usage

1. **Connect to Spotify**: Open Spotify app on your phone/computer
2. **Select device**: Choose "PiFi Speaker" from available devices
3. **Play music**: Song info will appear on display

### Controls

| Action | Function |
|--------|----------|
| Rotate encoder | Adjust volume (default mode) |
| Double-click | Switch between volume/scroll mode |
| Rotate (scroll mode) | Next/previous track |
| Single click | Play/pause |

## Verification Commands

```bash
# Check services
sudo systemctl status spotifyd mpris-spotifyd-pifi

# View logs
sudo journalctl -u mpris-spotifyd-pifi -f

# Check SPI
ls /dev/spi*

# Check audio
speaker-test -t wav -c 2

# List MPRIS players
busctl --user list | grep mpris
```

## Common Issues

### No display output
- Enable SPI: `sudo raspi-config` → Interface Options → SPI
- Check wiring (see WIRING.md)
- View logs: `sudo journalctl -u mpris-spotifyd-pifi -n 50`

### Spotifyd not connecting
- Verify credentials in `~/.config/spotifyd/spotifyd.conf`
- Check service: `sudo systemctl status spotifyd`
- Restart: `sudo systemctl restart spotifyd`

### Encoder not working
- Verify pin configuration in `~/.config/mpris-spotifyd-pifi.toml`
- Check connections (see WIRING.md)
- Test GPIO: `gpio readall`

### No audio
- Test: `speaker-test -t wav -c 2`
- Check ALSA: `alsamixer`
- Verify PiFi DAC overlay in `/boot/config.txt`

## Next Steps

- Read detailed [installation guide](INSTALACJA.md) (Polish)
- Check [wiring diagram](WIRING.md)
- Review [hardware display notes](HARDWARE_DISPLAY.md)
- See full [README](README.md)

## Getting Help

1. Check logs: `sudo journalctl -u mpris-spotifyd-pifi -f`
2. Verify wiring against WIRING.md
3. Read INSTALACJA.md for detailed troubleshooting
4. Open an issue on GitHub with logs attached
