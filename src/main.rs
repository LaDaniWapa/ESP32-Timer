pub mod app_state;
mod background;
mod clock_face;
pub mod commands;
mod display;
mod time;

use crate::app_state::AppStatus;
use crate::display::Screen;
use chrono::Timelike;
use commands::AppCommand;
use embedded_graphics::text::{Baseline, TextStyleBuilder};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb666,
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

const COLOR_1: Rgb666 = Rgb666::new(23, 20, 33); // #5c5185
const COLOR_2: Rgb666 = Rgb666::new(27, 30, 40); // #6c7ba1
const COLOR_3: Rgb666 = Rgb666::new(35, 44, 47); // #8eb4bd
const COLOR_4: Rgb666 = Rgb666::new(44, 52, 50); // #b2d4c9
const COLOR_5: Rgb666 = Rgb666::new(49, 59, 50); // #c7edc9
const COLOR_6: Rgb666 = Rgb666::new(57, 63, 57); // #e5ffe6

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

    let mut chrome_drawn = false;

    loop {
        if let Ok(new_status) = rx.try_recv() {
            status = new_status;

            match status {
                AppStatus::Connecting => screen.show_message("Waiting for connection...", 10, 10)?,
                AppStatus::Connected => screen.show_message("Connected!", 10, 10)?,
                AppStatus::SyncingTime => screen.show_message("Syncing time and date...", 10, 10)?,
                AppStatus::Ready => {
                    if !chrome_drawn {
                        screen.draw_clock_chrome()?;
                        chrome_drawn = true;
                    }
                }
            }
        }

        if status == AppStatus::Ready {
            let now = time::now_in_tz();
            let date = now.format("%a, %d %b").to_string();
            screen.update_clock(
                now.hour() as u8,
                now.minute() as u8,
                now.second() as u8,
                &date,
            )?;
        }

        FreeRtos::delay_ms(500);
    }
}
