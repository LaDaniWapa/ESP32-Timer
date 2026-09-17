use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb666,
    prelude::*,
    primitives::Rectangle,
    text::{Text, TextStyle},
};
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
        let data = vec![Rgb666::BLACK; WIDTH * HEIGHT];
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
        self.fbuf.fill_solid(&full_screen, Rgb666::BLACK)?;
        Ok(())
    }

    /// Clears a specific rectangular area of the screen (buffer only).
    pub fn clear_area(&mut self, x: i32, y: i32, width: u32, height: u32) -> anyhow::Result<()> {
        let area = Rectangle::new(Point::new(x, y), Size::new(width, height));
        self.fbuf.fill_solid(&area, Rgb666::BLACK)?;
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
}