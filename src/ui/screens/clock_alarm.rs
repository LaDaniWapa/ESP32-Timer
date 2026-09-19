use crate::input::ButtonEvent;
use crate::ui::display::Screen;
use crate::ui::screens::{AppScreen, ScreenLogic};

pub struct ClockAlarmScreen;

impl ClockAlarmScreen {
    pub fn new() -> Self {Self}
}

impl ScreenLogic for ClockAlarmScreen {
    fn draw_chrome(&mut self, display: &mut Screen) -> anyhow::Result<()> {
        todo!()
    }

    fn update(&mut self, display: &mut Screen) -> anyhow::Result<()> {
        todo!()
    }

    fn handle_input(&mut self, event: ButtonEvent, display: &mut Screen) -> anyhow::Result<Option<AppScreen>> {
        todo!()
    }
}