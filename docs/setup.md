# ESP32 Timer Setup


## 1. Install the toolchain (one-time setup)

```bash
# Rust + ESP tooling
cargo install espup
espup install
source $HOME/export-esp.sh   # you need to run this in every new terminal,
                              # or add it to your .bashrc/.zshrc
# Cargo tools for ESP-IDF
cargo install ldproxy espflash
```

## 2. Build and flash

Connect the ESP32-S3 via the USB-C port that acts as the UART/programmer then:

```bash
cd ESP32-Timer
cargo build --release
cargo run --release   # builds, flashes, and opens the serial monitor
```

If `cargo run` doesn't detect the port automatically, try:
```bash
espflash flash --monitor target/xtensa-esp32s3-espidf/release/ESP32-Timer
```

## 3. Wiring

The pins are defined at the top of `src/main.rs`. By default, they use the
SPI2 (FSPI) bus typical of ESP32-S3-DevKitC boards:

| Display          | ESP32-S3 |
|-------------------|----------|
| SCK               | GPIO12   |
| SDI (MOSI)        | GPIO11   |
| SDO (MISO)        | GPIO13   |
| CS                | GPIO10   |
| DC / RS           | GPIO9    |
| RESET             | GPIO8    |
| LED (backlight)   | 3.3V     |
| VCC               | 3.3V     |
| GND               | GND      |

If your wiring is different, change the `gpioX` numbers in `src/main.rs`.

## If the display doesn't turn on or looks wrong

- Garbled / shifted colors → try switching `MODE_0` to `MODE_3` in
  `spi_config`.
- Nothing on screen → check RESET and CS first, those are the most common
  culprits.
- All black → the backlight (LED) might not be powered.