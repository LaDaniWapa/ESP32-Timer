use esp_idf_svc::hal::gpio::{AnyIOPin, Input, PinDriver, Pull};
use std::time::{Duration, Instant};

const HOLD_THRESHOLD: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Mode,
    Snooze,
    Up,
    Down,
    Settings,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonEvent {
    Tap(Button),
    Hold(Button),
}

pub struct ButtonPoller {
    pins: [(Button, PinDriver<'static, AnyIOPin, Input>); 6],
    press_started: [Option<Instant>; 6],
}

impl ButtonPoller {
    pub fn new(
        mode: AnyIOPin,
        snooze: AnyIOPin,
        up: AnyIOPin,
        down: AnyIOPin,
        settings: AnyIOPin,
        off: AnyIOPin,
    ) -> anyhow::Result<Self> {
        fn make(pin:AnyIOPin) -> anyhow::Result<PinDriver<'static, AnyIOPin, Input>> {
            let mut driver = PinDriver::input(pin)?;
            driver.set_pull(Pull::Up)?; // buttons are wire to GND, so idle = high
            Ok(driver)
        }

        Ok(Self {
            pins: [
                (Button::Mode, make(mode)?),
                (Button::Snooze, make(snooze)?),
                (Button::Up, make(up)?),
                (Button::Down, make(down)?),
                (Button::Settings, make(settings)?),
                (Button::Off, make(off)?),
            ],
            press_started: [None; 6],
        })
    }

    /// Call every tick. Returns at most one event — the first button
    /// release detected this call.
    pub fn poll(&mut self) -> Option<ButtonEvent> {
        for i in 0..self.pins.len() {
            let (button, pin) = &mut self.pins[i];
            let pressed = pin.is_low(); // active-low

            match (pressed, self.press_started[i]) {
                (true, None) => {
                    self.press_started[i] = Some(Instant::now());
                }
                (false, Some(start)) => {
                    self.press_started[i] = None;
                    let event = if start.elapsed() >= HOLD_THRESHOLD {
                        ButtonEvent::Hold(*button)
                    } else {
                        ButtonEvent::Tap(*button)
                    };
                    return Some(event);
                }
                _ => {}
            }
        }
        None
    }
}