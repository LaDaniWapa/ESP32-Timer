use crate::{clock_face, COLOR_1, COLOR_2, COLOR_3, COLOR_5, COLOR_6};
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb666,
    prelude::*,
    primitives::Rectangle,
    text::{Text, TextStyle},
};
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics_framebuf::{backends::FrameBufferBackend, FrameBuf};
use mipidsi::interface::InterfacePixelFormat;

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 320;

/// Vec-backed framebuffer backend (lives in PSRAM).
pub struct VecBackend(pub Vec<Rgb666>);

impl FrameBufferBackend for VecBackend {
    type Color = Rgb666;

    fn set(&mut self, index: usize, color: Rgb666) {
        self.0[index] = color;
    }

    fn get(&self, index: usize) -> Rgb666 {
        self.0[index]
    }

    fn nr_elements(&self) -> usize {
        self.0.len()
    }
}

/// Owns the framebuffer and the physical display, plus the text styles,
/// so callers don't need to pass them into every drawing call.
pub struct Screen<DI, MODEL, RST>
where
    DI: mipidsi::interface::Interface,
    MODEL: mipidsi::models::Model<ColorFormat = Rgb666>,
    RST: embedded_hal::digital::OutputPin,
    Rgb666: InterfacePixelFormat<<DI as mipidsi::interface::Interface>::Word>,
{
    fbuf: FrameBuf<Rgb666, VecBackend>,
    display: mipidsi::Display<DI, MODEL, RST>,
    style: MonoTextStyle<'static, Rgb666>,
    text_style: TextStyle,
}

const STYLE5: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_5);
const STYLE6: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_6);


impl<DI, MODEL, RST> Screen<DI, MODEL, RST>
where
    DI: mipidsi::interface::Interface,
    MODEL: mipidsi::models::Model<ColorFormat = Rgb666>,
    RST: embedded_hal::digital::OutputPin,
    Rgb666: InterfacePixelFormat<<DI as mipidsi::interface::Interface>::Word>,
{
    pub fn new(
        display: mipidsi::Display<DI, MODEL, RST>,
        style: MonoTextStyle<'static, Rgb666>,
        text_style: TextStyle,
    ) -> Self {
        let data = vec![COLOR_6; WIDTH * HEIGHT];
        let fbuf = FrameBuf::new(VecBackend(data), WIDTH, HEIGHT);

        Self {
            fbuf,
            display,
            style,
            text_style,
        }
    }

    /// Clears the whole screen (buffer only — call `flush` or use
    /// `show_message`/`draw_text` to actually push it to the display).
    pub fn clear(&mut self) -> anyhow::Result<()> {
        let full_screen = Rectangle::new(Point::zero(), Size::new(WIDTH as u32, HEIGHT as u32));
        self.fbuf.fill_solid(&full_screen, COLOR_6)?;
        Ok(())
    }

    /// Clears a specific rectangular area of the screen (buffer only).
    pub fn clear_area(&mut self, x: i32, y: i32, width: u32, height: u32) -> anyhow::Result<()> {
        let area = Rectangle::new(Point::new(x, y), Size::new(width, height));
        self.fbuf.fill_solid(&area, COLOR_6)?;
        Ok(())
    }

    /// Sends the current framebuffer contents to the physical display.
    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.display
            .set_pixels(
                0,
                0,
                (WIDTH - 1) as u16,
                (HEIGHT - 1) as u16,
                self.fbuf.data.0.iter().copied(),
            )
            .map_err(|e| anyhow::anyhow!("Failed to flush the framebuffer: {:?}", e))
    }

    /// Sends only a rectangular region of the framebuffer to the physical display.
    pub fn flush_area(&mut self, x: i32, y: i32, w: u32, h: u32) -> anyhow::Result<()> {
        let x0 = x as u16;
        let y0 = y as u16;
        let x1 = (x + w as i32 - 1) as u16;
        let y1 = (y + h as i32 - 1) as u16;

        let data = &self.fbuf.data.0;
        let colors = (0..h)
            .flat_map(|row| {
                let py = y as usize + row as usize;
                let px0 = x as usize;
                (0..w as usize).map(move |col| data[py * WIDTH + px0 + col])
            })
            .collect::<Vec<_>>();

        self.display
            .set_pixels(x0, y0, x1, y1, colors.into_iter())
            .map_err(|e| anyhow::anyhow!("Failed to flush area: {:?}", e))
    }

    /// Clears the whole screen, draws text at the given position, and flushes.
    pub fn show_message(&mut self, text: &str, x: i32, y: i32) -> anyhow::Result<()> {
        self.clear()?;
        Text::with_text_style(text, Point::new(x, y), self.style, self.text_style)
            .draw(&mut self.fbuf)?;
        self.flush()
    }

    /// Draws text without clearing first, then flushes. Use this after a
    /// manual `clear_area` (e.g. redrawing just the clock).
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32) -> anyhow::Result<()> {
        Text::with_text_style(text, Point::new(x, y), self.style, self.text_style)
            .draw(&mut self.fbuf)?;
        self.flush()
    }

    /// Draws text with custom style without clearing first, then flushes. Use this after a
    /// manual `clear_area` (e.g. redrawing just the clock).
    pub fn draw_styled_text(&mut self, text: &str, x: i32, y: i32, style: MonoTextStyle<Rgb666>) -> anyhow::Result<()> {
        Text::with_text_style(text, Point::new(x, y), style, self.text_style)
            .draw(&mut self.fbuf)?;
        self.flush()
    }

    pub fn draw_fake_button(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        border_color: Rgb666,
        bg_color: Rgb666,
        text: &str,
        text_style: MonoTextStyle<'static, Rgb666>,
    ) -> anyhow::Result<()> {
        const BORDER: u32 = 2;

        // Border
        self.draw_rect(x, y, w, h, border_color)?;
        // Fill (inset by the border thickness on every side)
        self.draw_rect(
            x + BORDER as i32,
            y + BORDER as i32,
            w - BORDER * 2,
            h - BORDER * 2,
            bg_color,
        )?;

        // Center the text inside the button
        let char_size = text_style.font.character_size;
        let text_w = char_size.width * text.len() as u32;
        let text_h = char_size.height;
        let text_x = x + (w as i32 - text_w as i32) / 2;
        let text_y = y + (h as i32 - text_h as i32) / 2;

        self.draw_styled_text(text, text_x, text_y, text_style)
    }

    /// Draws the static chrome (bars + buttons) once. Call this only when
    /// entering Clock_Screen, not on every tick.
    pub fn draw_clock_chrome(&mut self) -> anyhow::Result<()> {
        self.clear()?;

        self.draw_rect(0, 0, WIDTH as u32, 30, COLOR_1)?;
        self.draw_rect(0, (HEIGHT - 36) as i32, WIDTH as u32, 36, COLOR_1)?;
        self.draw_fake_button(76, (HEIGHT - 31) as i32, 160, 26, COLOR_6, COLOR_2, "CLOCK", STYLE5)?;
        self.draw_fake_button(244, (HEIGHT - 31) as i32, 160, 26, COLOR_6, COLOR_3, "POMODORO", STYLE6)?;

        self.flush()
    }

    pub fn update_clock(&mut self, hour: u8, minute: u8, second: u8, date: &str) -> anyhow::Result<()> {
        let time_x = (WIDTH as i32 - clock_face::TIME_WIDTH as i32) / 2;
        let (dyn_x, dyn_y, dyn_w, dyn_h) = (0, 30, WIDTH as u32, HEIGHT as u32 - 66);

        self.clear_area(dyn_x, dyn_y, dyn_w, dyn_h)?;

        clock_face::draw_time(&mut self.fbuf, hour, minute, time_x, 60, COLOR_1)?;

        self.draw_styled_text(date, 10, 6, STYLE5)?;

        self.flush_area(dyn_x, dyn_y, dyn_w, dyn_h)
    }

    pub fn draw_rect(&mut self, x:i32, y:i32, w:u32, h:u32, color: Rgb666) -> anyhow::Result<()> {
        let area = Rectangle::new(Point::new(x, y), Size::new(w, h));
        self.fbuf.fill_solid(&area, color)?;
        Ok(())
    }
}
