use chrono::{DateTime, Utc};
use chrono_tz::{Europe::Madrid, Tz};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use std::time::SystemTime;

pub fn start_sync () -> anyhow::Result<EspSntp<'static>> {
    Ok(EspSntp::new_default()?)
}

pub fn wait_for_sync(ntp: &EspSntp) {
    while ntp.get_sync_status() != SyncStatus::Completed {
        FreeRtos::delay_ms(200)
    }
}

pub fn now_in_tz() ->DateTime<Tz> {
    let utc_now: DateTime<Utc> = SystemTime::now().into();
    utc_now.with_timezone(&Madrid)
}

pub fn formatted_now() -> String {
    now_in_tz().format("%d/%m/%Y %H:%M:%S").to_string()
}