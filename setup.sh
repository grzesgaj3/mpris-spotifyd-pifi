#!/bin/bash
# Setup script for MPRIS Spotifyd PiFi on Raspberry Pi Zero

set -e

echo "=== MPRIS Spotifyd PiFi Setup ==="
echo ""

# Check if running on Raspberry Pi
if [ ! -f /proc/device-tree/model ]; then
    echo "Warning: This script is designed for Raspberry Pi"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if running as pi user
if [ "$USER" != "pi" ]; then
    echo "Warning: This script should be run as 'pi' user"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo "Step 1: Checking dependencies..."
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✓ Rust is already installed"
fi

echo ""
echo "Step 2: Enabling SPI interface..."
if ! grep -q "^dtparam=spi=on" /boot/config.txt; then
    echo "Enabling SPI in /boot/config.txt (requires sudo)"
    sudo sed -i 's/^#dtparam=spi=on/dtparam=spi=on/' /boot/config.txt
    if ! grep -q "^dtparam=spi=on" /boot/config.txt; then
        echo "dtparam=spi=on" | sudo tee -a /boot/config.txt > /dev/null
    fi
    echo "✓ SPI enabled (requires reboot)"
    NEEDS_REBOOT=true
else
    echo "✓ SPI already enabled"
fi

echo ""
echo "Step 3: Building application..."
cargo build --release

echo ""
echo "Step 4: Installing binary..."
sudo cp target/release/mpris-spotifyd-pifi /usr/local/bin/
sudo chmod +x /usr/local/bin/mpris-spotifyd-pifi
echo "✓ Binary installed to /usr/local/bin/mpris-spotifyd-pifi"

echo ""
echo "Step 5: Installing configuration..."
mkdir -p ~/.config
if [ ! -f ~/.config/mpris-spotifyd-pifi.toml ]; then
    cp config.toml ~/.config/mpris-spotifyd-pifi.toml
    echo "✓ Configuration installed to ~/.config/mpris-spotifyd-pifi.toml"
else
    echo "✓ Configuration already exists at ~/.config/mpris-spotifyd-pifi.toml"
fi

echo ""
echo "Step 6: Installing systemd service..."
sudo cp mpris-spotifyd-pifi.service /etc/systemd/system/
sudo systemctl daemon-reload
echo "✓ Systemd service installed"

echo ""
echo "=== Setup Complete ==="
echo ""
echo "To start the service now:"
echo "  sudo systemctl start mpris-spotifyd-pifi"
echo ""
echo "To enable autostart on boot:"
echo "  sudo systemctl enable mpris-spotifyd-pifi"
echo ""
echo "To view logs:"
echo "  sudo journalctl -u mpris-spotifyd-pifi -f"
echo ""

if [ "$NEEDS_REBOOT" = true ]; then
    echo "⚠️  REBOOT REQUIRED to enable SPI interface"
    echo "   Run: sudo reboot"
    echo ""
fi

echo "Note: Make sure Spotifyd is installed and configured"
echo "      See README.md for Spotifyd setup instructions"
