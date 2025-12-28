# Instrukcja instalacji MPRIS Spotifyd PiFi

Kompletny przewodnik instalacji dla Raspberry Pi Zero WH z nakładką PiFi DAC+.

## Wymagania

- Raspberry Pi Zero WH z zainstalowanym Raspberry Pi OS Lite
- Karta microSD (minimum 8GB)
- Połączenie z internetem (WiFi lub Ethernet przez USB)
- Nakładka PiFi DAC+
- Wyświetlacz OLED SSD1306 128x32
- Enkoder obrotowy z przyciskiem

## Krok 1: Przygotowanie systemu

```bash
# Aktualizacja systemu
sudo apt update
sudo apt upgrade -y

# Instalacja niezbędnych pakietów
sudo apt install -y git curl build-essential pkg-config \
    libdbus-1-dev libasound2-dev

# Włączenie SPI dla wyświetlacza
sudo raspi-config
# Wybierz: Interface Options → SPI → Yes
# Wyjdź i zrestartuj: sudo reboot
```

## Krok 2: Instalacja Rust

```bash
# Instalacja Rust przez rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Wybierz opcję 1 (domyślna instalacja)
# Po instalacji:
source $HOME/.cargo/env

# Sprawdzenie instalacji
rustc --version
cargo --version
```

## Krok 3: Konfiguracja PiFi DAC+

```bash
# Edycja pliku konfiguracyjnego boot
sudo nano /boot/config.txt

# Dodaj na końcu pliku:
dtoverlay=hifiberry-dac
force_eeprom_read=0

# Zapisz (Ctrl+O, Enter) i wyjdź (Ctrl+X)

# Edycja konfiguracji ALSA
sudo nano /etc/asound.conf

# Wklej:
pcm.!default {
    type hw
    card 0
}
ctl.!default {
    type hw
    card 0
}

# Zapisz i wyjdź
# Restart systemu
sudo reboot
```

## Krok 4: Instalacja Spotifyd

```bash
# Pobieranie Spotifyd
cd ~
wget https://github.com/Spotifyd/spotifyd/releases/latest/download/spotifyd-linux-armhf-default.tar.gz

# Rozpakowanie
tar xzf spotifyd-linux-armhf-default.tar.gz

# Instalacja
sudo mv spotifyd /usr/local/bin/
sudo chmod +x /usr/local/bin/spotifyd

# Utworzenie katalogu konfiguracyjnego
mkdir -p ~/.config/spotifyd

# Utworzenie pliku konfiguracyjnego
nano ~/.config/spotifyd/spotifyd.conf
```

Wklej do pliku konfiguracyjnego (zamień dane logowania):

```toml
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
initial_volume = "75"
volume_controller = "alsa"
mixer = "PCM"
```

Zapisz plik (Ctrl+O, Enter, Ctrl+X).

## Krok 5: Konfiguracja Spotifyd jako usługa systemd

```bash
# Utworzenie pliku usługi
sudo nano /etc/systemd/system/spotifyd.service
```

Wklej:

```ini
[Unit]
Description=Spotifyd Spotify Daemon
After=network-online.target sound.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/spotifyd --no-daemon
Restart=always
RestartSec=12
User=pi

[Install]
WantedBy=multi-user.target
```

Zapisz i włącz usługę:

```bash
sudo systemctl daemon-reload
sudo systemctl enable spotifyd
sudo systemctl start spotifyd

# Sprawdzenie statusu
sudo systemctl status spotifyd
```

## Krok 6: Instalacja aplikacji MPRIS Spotifyd PiFi

```bash
# Klonowanie repozytorium
cd ~
git clone https://github.com/grzesgaj3/mpris-spotifyd-pifi.git
cd mpris-spotifyd-pifi

# Kompilacja (może potrwać 10-30 minut na Pi Zero)
cargo build --release

# Instalacja
sudo cp target/release/mpris-spotifyd-pifi /usr/local/bin/
sudo chmod +x /usr/local/bin/mpris-spotifyd-pifi

# Kopiowanie konfiguracji
mkdir -p ~/.config
cp config.toml ~/.config/mpris-spotifyd-pifi.toml
```

## Krok 7: Konfiguracja pinów GPIO

Edytuj konfigurację jeśli używasz innych pinów:

```bash
nano ~/.config/mpris-spotifyd-pifi.toml
```

Domyślna konfiguracja:
```toml
[display]
spi_device = "/dev/spidev0.0"
dc_pin = 24    # GPIO24 dla DC
rst_pin = 25   # GPIO25 dla RST
update_interval_ms = 500

[rotary]
clk_pin = 17   # GPIO17 dla CLK
dt_pin = 27    # GPIO27 dla DT
sw_pin = 22    # GPIO22 dla SW
volume_step = 0.05

[mpris]
reconnect_interval_ms = 5000
```

## Krok 8: Konfiguracja jako usługa systemd

```bash
# Kopiowanie pliku usługi
sudo cp mpris-spotifyd-pifi.service /etc/systemd/system/

# Włączenie i uruchomienie
sudo systemctl daemon-reload
sudo systemctl enable mpris-spotifyd-pifi
sudo systemctl start mpris-spotifyd-pifi

# Sprawdzenie statusu
sudo systemctl status mpris-spotifyd-pifi

# Podgląd logów na żywo
sudo journalctl -u mpris-spotifyd-pifi -f
```

## Testowanie

1. Otwórz aplikację Spotify na telefonie/komputerze
2. Wybierz urządzenie "PiFi Speaker" z listy dostępnych urządzeń
3. Rozpocznij odtwarzanie - informacje powinny pojawić się na wyświetlaczu
4. Przetestuj enkoder obrotowy:
   - Obrót: zmiana głośności
   - Podwójne kliknięcie: przełączenie do trybu przewijania
   - Obrót w trybie przewijania: zmiana utworu
   - Pojedyncze kliknięcie: pauza/wznowienie

## Rozwiązywanie problemów

### Spotifyd nie łączy się z Spotify

```bash
# Sprawdź logi
sudo journalctl -u spotifyd -n 50

# Sprawdź czy daemon działa
ps aux | grep spotifyd

# Restart usługi
sudo systemctl restart spotifyd
```

### Wyświetlacz nie działa

```bash
# Sprawdź czy SPI jest włączone
ls -l /dev/spi*

# Powinno pokazać: /dev/spidev0.0 i /dev/spidev0.1

# Sprawdź logi aplikacji
sudo journalctl -u mpris-spotifyd-pifi -n 50
```

### Enkoder nie reaguje

```bash
# Sprawdź piny GPIO
gpio readall

# Sprawdź konfigurację
cat ~/.config/mpris-spotifyd-pifi.toml
```

### Aplikacja nie widzi MPRIS

```bash
# Sprawdź czy Spotifyd publikuje MPRIS
dbus-send --print-reply --session \
  --dest=org.freedesktop.DBus \
  /org/freedesktop/DBus \
  org.freedesktop.DBus.ListNames

# Powinna być widoczna nazwa zawierająca "spotify" lub "mpris"
```

## Automatyczne uruchamianie przy starcie

Usługi systemd są już skonfigurowane do automatycznego startu.
Sprawdź status:

```bash
sudo systemctl is-enabled spotifyd
sudo systemctl is-enabled mpris-spotifyd-pifi
```

Oba powinny pokazać: `enabled`

## Aktualizacja aplikacji

```bash
cd ~/mpris-spotifyd-pifi
git pull
cargo build --release
sudo systemctl stop mpris-spotifyd-pifi
sudo cp target/release/mpris-spotifyd-pifi /usr/local/bin/
sudo systemctl start mpris-spotifyd-pifi
```

## Wsparcie

W przypadku problemów:
1. Sprawdź logi: `sudo journalctl -u mpris-spotifyd-pifi -f`
2. Sprawdź połączenia GPIO zgodnie z plikiem WIRING.md
3. Zgłoś issue na GitHub z logami

## Użyteczne komendy

```bash
# Restart wszystkiego
sudo systemctl restart spotifyd mpris-spotifyd-pifi

# Zatrzymanie wszystkiego
sudo systemctl stop spotifyd mpris-spotifyd-pifi

# Wyłączenie autostartu
sudo systemctl disable mpris-spotifyd-pifi

# Włączenie autostartu
sudo systemctl enable mpris-spotifyd-pifi

# Test głośności ALSA
alsamixer

# Test odtwarzania dźwięku
speaker-test -t wav -c 2
```
