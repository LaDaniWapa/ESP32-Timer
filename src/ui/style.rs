use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb666;

pub const COLOR_1: Rgb666 = Rgb666::new(23, 20, 33); // #5c5185
pub const COLOR_2: Rgb666 = Rgb666::new(27, 30, 40); // #6c7ba1
pub const COLOR_3: Rgb666 = Rgb666::new(35, 44, 47); // #8eb4bd
pub const COLOR_4: Rgb666 = Rgb666::new(44, 52, 50); // #b2d4c9
pub const COLOR_5: Rgb666 = Rgb666::new(49, 59, 50); // #c7edc9
pub const COLOR_6: Rgb666 = Rgb666::new(57, 63, 57); // #e5ffe6


pub const STYLE1: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_1);
pub const STYLE2: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_1);
pub const STYLE3: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_1);
pub const STYLE4: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_4);
pub const STYLE5: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_5);
pub const STYLE6: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_10X20, COLOR_6);