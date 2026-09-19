mod app_state;
mod background;
mod commands;
mod input;
mod time;
mod ui;

use crate::app_state::AppStatus;
use crate::input::ButtonPoller;
use crate::ui::screens::clock_screen::ClockScreen;
use crate::ui::screens::AppScreen;
use crate::ui::{display::Screen, style::COLOR_1};
use chrono::Timelike;
use commands::AppCommand;
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::text::{Baseline, TextStyleBuilder};
use embedded_hal::spi::MODE_0;
use esp_idf_svc::hal::units::MegaHertz;
use esp_idf_svc::hal::{
    delay::{Ets, FreeRtos},
    gpio::PinDriver,
    peripherals::Peripherals,
    spi::{config::Config, SpiDeviceDriver, SpiDriverConfig},
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use mipidsi::options::{Orientation, Rotation};
use mipidsi::{interface::SpiInterface, models::ILI9488Rgb666, Builder};
use std::sync::mpsc;
use std::thread;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi = background::setup(peripherals.modem, sysloop, nvs)?;

    let pins = peripherals.pins;

    let mut buttons = ButtonPoller::new(
        pins.gpio4.into(),
        pins.gpio5.into(),
        pins.gpio6.into(),
        pins.gpio7.into(),
        pins.gpio15.into(),
        pins.gpio16.into(),
    )?;

    let sclk = pins.gpio12;
    let sda = pins.gpio11;
    let sdi = pins.gpio13;
    let cs = pins.gpio10;
    let dc = PinDriver::output(pins.gpio9)?;
    let rst = PinDriver::output(pins.gpio8)?;

    let spi_config = Config::new()
        .baudrate(MegaHertz(40).into())
        .data_mode(MODE_0);

    let spi = SpiDeviceDriver::new_single(
        peripherals.spi2,
        sclk,
        sda,
        Some(sdi),
        Some(cs),
        &SpiDriverConfig::new(),
        &spi_config,
    )?;

    let buffer: &'static mut [u8;512] = Box::leak(Box::new([0u8; 512]));
    let di = SpiInterface::new(spi, dc, buffer);

    let mut delay = Ets;
    let display = Builder::new(ILI9488Rgb666, di)
        .reset_pin(rst)
        .orientation(
            Orientation::new()
                .rotate(Rotation::Deg270)
                .flip_horizontal(),
        )
        .init(&mut delay)
        .map_err(|e| anyhow::anyhow!("Error while trying to init screen: {:?}", e))?;

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    let style = MonoTextStyle::new(&FONT_10X20, COLOR_1);
    let mut screen = Screen::new(display, style, text_style);

    let (tx, _rx) = mpsc::channel::<AppStatus>();
    let (_tx_commands, rx_commands) = mpsc::channel::<AppCommand>();

    // --- Network Thread ---
    thread::Builder::new().stack_size(8192).spawn(move || {
        if let Err(e) = background::run(wifi, tx, rx_commands) {
            log::error!("Error on background thread: {:?}", e);
        }
    })?;

    // --- UI Loop ---
    let mut _status = AppStatus::Connecting;
    screen.show_message("Waiting for connection...", 10, 10)?;

    screen.clear()?;

    let mut current_screen = AppScreen::Clock(ClockScreen::new());
    current_screen.draw_chrome(&mut screen)?;

    loop {
        if let Some(event) = buttons.poll() {
            if let Some(next) = current_screen.handle_input(event, &mut screen)? {
                current_screen = next;
                current_screen.draw_chrome(&mut screen)?;
            }
        }

        current_screen.update(&mut screen)?;
        FreeRtos::delay_ms(500);
    }
}
