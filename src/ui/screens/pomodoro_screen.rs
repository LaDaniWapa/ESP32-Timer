use crate::input::ButtonEvent;
use crate::ui::display::Screen;
use crate::ui::screens::{AppScreen, ScreenLogic};

pub struct PomodoroScreen;

impl PomodoroScreen {
    pub fn new() -> Self {Self}
}

impl ScreenLogic for PomodoroScreen {
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