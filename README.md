# ESP32 Timer

A desk clock built on an ESP32-S3, written in Rust.

## Hardware

- ESP32-S3 N16R8 (16MB flash, 8MB octal PSRAM)
- 3.5" TFT SPI display, 480x320, ILI9488 controller

## Stack

- Rust in `std` mode, on top of ESP-IDF (not `no_std`/bare-metal)
- [`mipidsi`](https://crates.io/crates/mipidsi) as the display driver
- [`embedded-graphics`](https://crates.io/crates/embedded-graphics) for drawing
- [`embedded-graphics-framebuf`](https://crates.io/crates/embedded-graphics-framebuf)
  for an in-memory framebuffer, to avoid flicker when redrawing
- WiFi + SNTP for clock sync, converted to Europe/Madrid local time
  (DST-aware) via [`chrono-tz`](https://crates.io/crates/chrono-tz)
- WiFi connect and NTP sync run on a background thread, so the UI thread
  never blocks while drawing the display

## Project layout

- `src/display.rs` — `Screen` struct: owns the framebuffer and the
  physical display, exposes drawing methods
- `src/wifi.rs` — WiFi client setup
- `src/time.rs` — NTP sync and local time formatting
- `src/background.rs` — background thread: connects WiFi, syncs NTP,
  and will host future timers (pomodoro, alarms)
- `src/app_state.rs` / `src/commands.rs` — the two-way channel contract
  between the UI thread and the background thread

## Building

See the toolchain setup and wiring notes in [setup.md](docs/setup.md)
<!-- ajusta o quita esta línea si no vas a tener ese archivo -->