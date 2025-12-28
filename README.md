# MPRIS Spotifyd PiFi

Aplikacja w Rust dla Raspberry Pi Zero WH z nakładką PiFi DAC+ umożliwiająca wyświetlanie informacji o aktualnie odtwarzanym utworze ze Spotify Connect oraz kontrolę odtwarzania.

## Funkcjonalność

- **Wyświetlanie informacji o utworze** na ekranie SSD1306 128x32 (SPI):
  - Tytuł utworu
  - Wykonawca
  - Pasek postępu odtwarzania
- **Obsługa enkodera obrotowego**:
  - **Tryb głośności** (domyślny): obrót zmienia głośność
  - **Tryb przewijania**: obrót zmienia utwór (do przodu/do tyłu)
  - Podwójne kliknięcie przełącza między trybami
  - Pojedyncze kliknięcie: pauza/wznowienie odtwarzania
- **Integracja MPRIS**: odczyt informacji z dowolnego odtwarzacza obsługującego MPRIS (Spotify, Spotifyd, VLC, itp.)

## Wymagania sprzętowe

- Raspberry Pi Zero WH
- PiFi DAC+ (lub inna nakładka audio)
- Wyświetlacz OLED SSD1306 128x32 (interfejs SPI)
- Enkoder obrotowy z przyciskiem

## Połączenia GPIO

### Wyświetlacz SSD1306 (SPI)
- **VCC** → 3.3V
- **GND** → GND
- **SCL** → GPIO 11 (SCLK)
- **SDA** → GPIO 10 (MOSI)
- **DC** → GPIO 24 (konfigurowalny)
- **RST** → GPIO 25 (konfigurowalny)
- **CS** → GPIO 8 (CE0)

### Enkoder obrotowy
- **CLK** → GPIO 17 (konfigurowalny)
- **DT** → GPIO 27 (konfigurowalny)
- **SW** → GPIO 22 (konfigurowalny)
- **+** → 3.3V
- **GND** → GND

## Instalacja

### 1. Przygotowanie systemu

```bash
# Aktualizacja systemu
sudo apt update && sudo apt upgrade -y

# Instalacja zależności
sudo apt install -y git curl build-essential libasound2-dev

# Włączenie SPI
sudo raspi-config
# Interface Options → SPI → Yes
```

### 2. Instalacja Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Instalacja Spotifyd

```bash
# Instalacja spotifyd (Spotify daemon dla Linux)
curl -sL https://github.com/Spotifyd/spotifyd/releases/latest/download/spotifyd-linux-armhf-default.tar.gz | tar xz
sudo mv spotifyd /usr/local/bin/

# Konfiguracja spotifyd
mkdir -p ~/.config/spotifyd
cat > ~/.config/spotifyd/spotifyd.conf << 'EOF'
[global]
username = "TWOJ_LOGIN_SPOTIFY"
password = "TWOJE_HASLO"
backend = "alsa"
device_name = "PiFi Speaker"
bitrate = 320
cache_path = "/home/pi/.cache/spotifyd"
volume_normalisation = true
normalisation_pregain = -10
device_type = "speaker"
EOF

# Utworzenie usługi systemd dla spotifyd
sudo cat > /etc/systemd/system/spotifyd.service << 'EOF'
[Unit]
Description=Spotifyd
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/spotifyd --no-daemon
Restart=always
RestartSec=12
User=pi

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable spotifyd
sudo systemctl start spotifyd
```

### 4. Kompilacja i instalacja aplikacji

```bash
# Sklonowanie repozytorium
git clone https://github.com/grzesgaj3/mpris-spotifyd-pifi.git
cd mpris-spotifyd-pifi

# Kompilacja (tryb release dla optymalizacji)
cargo build --release

# Instalacja
sudo cp target/release/mpris-spotifyd-pifi /usr/local/bin/

# Kopiowanie pliku konfiguracyjnego
cp config.toml ~/.config/mpris-spotifyd-pifi.toml
```

### 5. Konfiguracja jako usługa systemd

```bash
sudo cat > /etc/systemd/system/mpris-spotifyd-pifi.service << 'EOF'
[Unit]
Description=MPRIS Spotifyd PiFi Display
After=spotifyd.service
Wants=spotifyd.service

[Service]
Type=simple
ExecStart=/usr/local/bin/mpris-spotifyd-pifi
WorkingDirectory=/home/pi/.config
Restart=always
RestartSec=5
User=pi
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable mpris-spotifyd-pifi
sudo systemctl start mpris-spotifyd-pifi
```

## Konfiguracja

Edytuj plik `config.toml` aby dostosować pin GPIO i inne ustawienia:

```toml
[display]
spi_device = "/dev/spidev0.0"
dc_pin = 24
rst_pin = 25
update_interval_ms = 500

[rotary]
clk_pin = 17
dt_pin = 27
sw_pin = 22
volume_step = 0.05

[mpris]
reconnect_interval_ms = 5000
```

## Użytkowanie

Po uruchomieniu aplikacja automatycznie:
1. Inicjalizuje wyświetlacz SSD1306
2. Łączy się z odtwarzaczem MPRIS (Spotifyd)
3. Wyświetla informacje o aktualnie granym utworze

### Sterowanie enkoderem

- **Obrót w prawo/lewo** (tryb głośności): zwiększ/zmniejsz głośność
- **Obrót w prawo/lewo** (tryb przewijania): następny/poprzedni utwór
- **Podwójne kliknięcie**: przełącz między trybem głośności a przewijania
- **Pojedyncze kliknięcie**: pauza/wznowienie

## Rozwój i debugging

```bash
# Uruchomienie w trybie debug z logami
RUST_LOG=debug cargo run

# Sprawdzenie logów usługi
sudo journalctl -u mpris-spotifyd-pifi -f

# Sprawdzenie statusu
sudo systemctl status mpris-spotifyd-pifi
```

## Kompilacja krzyżowa (opcjonalnie)

Jeśli chcesz kompilować na szybszym komputerze:

```bash
# Na komputerze deweloperskim
rustup target add armv6-unknown-linux-gnueabihf

# Instalacja cross
cargo install cross

# Kompilacja
cross build --release --target armv6-unknown-linux-gnueabihf

# Skopiowanie na Raspberry Pi
scp target/armv6-unknown-linux-gnueabihf/release/mpris-spotifyd-pifi pi@raspberrypi.local:~/
```

## Rozwiązywanie problemów

### Wyświetlacz nie działa
- Sprawdź czy SPI jest włączone: `ls /dev/spi*`
- Sprawdź połączenia GPIO
- Sprawdź logi: `sudo journalctl -u mpris-spotifyd-pifi -f`

### Brak połączenia z MPRIS
- Sprawdź czy Spotifyd działa: `systemctl status spotifyd`
- Sprawdź logi Spotifyd: `journalctl -u spotifyd -f`
- Sprawdź czy coś jest odtwarzane w Spotify Connect

### Enkoder nie reaguje
- Sprawdź połączenia GPIO
- Sprawdź piny w konfiguracji
- Przetestuj enkoder: `gpio readall`

## Licencja

MIT

## Autor

Created for Raspberry Pi Zero WH with PiFi DAC+ integration
