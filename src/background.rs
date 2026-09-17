use crate::app_state::AppStatus;
use crate::commands::AppCommand;
use crate::time;
use embedded_hal::delay::DelayNs;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::modem::Modem,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};
use std::sync::mpsc::{
    Receiver,
    RecvTimeoutError::{Disconnected, Timeout},
    Sender,
};
use std::time::Duration;

/// Configs Wi-Fi client
pub fn setup(
    modem: Modem,
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
) -> anyhow::Result<BlockingWifi<EspWifi<'static>>> {
    let mut wifi = BlockingWifi::wrap(EspWifi::new(modem, sysloop.clone(), Some(nvs))?, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "WiFi Peste".try_into().unwrap(),
        password: "esafloraxunga43317u7".try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    Ok(wifi)
}

/// Connects to WiFi, syncs with NTP and reports through 'tx'
/// Blocks because it runs on its own thread
pub fn run(
    mut wifi: BlockingWifi<EspWifi<'static>>,
    tx: Sender<AppStatus>,
    rx_commands: Receiver<AppCommand>,
) -> anyhow::Result<()> {
    wifi.start()?;
    wifi.connect()?;

    while !wifi.is_connected()? {
        FreeRtos.delay_ms(200);
    }
    tx.send(AppStatus::Connected).ok();

    let ntp = time::start_sync()?;
    tx.send(AppStatus::SyncingTime).ok();

    time::wait_for_sync(&ntp);
    tx.send(AppStatus::Ready).ok();

    // --- Ticking loop ---
    loop {
        // waits 200ms for a command
        match rx_commands.recv_timeout(Duration::from_millis(200)) {
            Ok(_cmd) => {
                // match AppCommand
            }
            Err(Timeout) => {
                // Check for alarms that should go off
            }
            Err(Disconnected) => {
                // UI closed the channel, should happen
                break;
            }
        }
    }

    Ok(())
}
