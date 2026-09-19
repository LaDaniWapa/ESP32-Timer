use crate::input::ButtonEvent;
use crate::ui::display::Screen;
use crate::ui::screens::alarms_screen::AlarmsScreen;
use crate::ui::screens::clock_alarm::ClockAlarmScreen;
use crate::ui::screens::clock_screen::ClockScreen;
use crate::ui::screens::global_settings::GlobalSettingsScreen;
use crate::ui::screens::pomodoro_alarm::PomodoroAlarmScreen;
use crate::ui::screens::pomodoro_screen::PomodoroScreen;
use crate::ui::screens::pomodoro_settings::PomodoroSettingsScreen;

pub mod alarms_screen;
pub mod clock_alarm;
pub mod clock_screen;
pub mod global_settings;
pub mod pomodoro_alarm;
pub mod pomodoro_screen;
pub mod pomodoro_settings;

pub enum AppScreen {
    Clock(ClockScreen),
    ClockAlarm(ClockAlarmScreen),
    Pomodoro(PomodoroScreen),
    PomodoroAlarm(PomodoroAlarmScreen),
    PomodoroSettings(PomodoroSettingsScreen),
    Alarms(AlarmsScreen),
    GlobalSettings(GlobalSettingsScreen),
}

pub trait ScreenLogic {
    fn draw_chrome(&mut self, display: &mut Screen) -> anyhow::Result<()>;
    fn update(&mut self, display: &mut Screen) -> anyhow::Result<()>;
    fn handle_input(
        &mut self,
        event: ButtonEvent,
        display: &mut Screen,
    ) -> anyhow::Result<Option<AppScreen>>;
}

impl AppScreen {
    pub fn draw_chrome(&mut self, display: &mut Screen) -> anyhow::Result<()> {
        match self {
            AppScreen::Clock(s) => s.draw_chrome(display),
            AppScreen::ClockAlarm(s) => s.draw_chrome(display),
            AppScreen::Pomodoro(s) => s.draw_chrome(display),
            AppScreen::PomodoroAlarm(s) => s.draw_chrome(display),
            AppScreen::PomodoroSettings(s) => s.draw_chrome(display),
            AppScreen::Alarms(s) => s.draw_chrome(display),
            AppScreen::GlobalSettings(s) => s.draw_chrome(display),
        }
    }

    pub fn update(&mut self, display: &mut Screen) -> anyhow::Result<()> {
        match self {
            AppScreen::Clock(s) => s.update(display),
            AppScreen::ClockAlarm(s) => s.update(display),
            AppScreen::Pomodoro(s) => s.update(display),
            AppScreen::PomodoroAlarm(s) => s.update(display),
            AppScreen::PomodoroSettings(s) => s.update(display),
            AppScreen::Alarms(s) => s.update(display),
            AppScreen::GlobalSettings(s) => s.update(display),
        }
    }

    pub fn handle_input(
        &mut self,
        event: ButtonEvent,
        display: &mut Screen,
    ) -> anyhow::Result<Option<AppScreen>> {
        match self {
            AppScreen::Clock(s) => s.handle_input(event, display),
            AppScreen::ClockAlarm(s) => s.handle_input(event, display),
            AppScreen::Pomodoro(s) => s.handle_input(event, display),
            AppScreen::PomodoroAlarm(s) => s.handle_input(event, display),
            AppScreen::PomodoroSettings(s) => s.handle_input(event, display),
            AppScreen::Alarms(s) => s.handle_input(event, display),
            AppScreen::GlobalSettings(s) => s.handle_input(event, display),
        }
    }
}
