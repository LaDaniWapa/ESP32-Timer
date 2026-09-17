pub mod app_state;
mod background;
pub mod commands;
mod display;
mod time;
use crate::app_state::AppStatus;
use crate::display::Screen;
use commands::AppCommand;
use embedded_graphics::text::{Baseline, TextStyleBuilder};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb666,
    prelude::*,
};
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

    let mut buffer = [0u8; 512];
    let di = SpiInterface::new(spi, dc, &mut buffer);

    let mut delay = Ets;
    let display = Builder::new(ILI9488Rgb666, di)
        .reset_pin(rst)
        .orientation(Orientation::new().rotate(Rotation::Deg90).flip_horizontal())
        .init(&mut delay)
        .map_err(|e| anyhow::anyhow!("Error while trying to init screen: {:?}", e))?;

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    let style = MonoTextStyle::new(&FONT_10X20, Rgb666::WHITE);
    let mut screen = Screen::new(display, style, text_style);

    let (tx, rx) = mpsc::channel::<AppStatus>();
    let (_tx_commands, rx_commands) = mpsc::channel::<AppCommand>();

    // --- Network Thread ---
    thread::Builder::new().stack_size(8192).spawn(move || {
        if let Err(e) = background::run(wifi, tx, rx_commands) {
            log::error!("Error on background thread: {:?}", e);
        }
    })?;

    // --- UI Loop ---
    let mut status = AppStatus::Connecting;
    screen.show_message("Waiting for connection...", 10, 10)?;

    screen.clear()?;

    loop {
        if let Ok(new_status) = rx.try_recv() {
            status = new_status;

            let message = match status {
                AppStatus::Connecting => "Waiting for connection...",
                AppStatus::Connected => "Connected!",
                AppStatus::SyncingTime => "Syncing time and date...",
                AppStatus::Ready => "",
            };

            if status == AppStatus::Ready {
                screen.clear()?;
                screen.flush()?;
            } else {
                screen.show_message(message, 10, 10)?;
            }
        }

        if status == AppStatus::Ready {
            let formatted = time::formatted_now();

            screen.clear_area(140,145,200,20)?;
            screen.draw_text(&formatted,145,150)?;
            screen.flush()?;
        }

        FreeRtos::delay_ms(500);
    }
}
