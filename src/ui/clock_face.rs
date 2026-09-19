use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

const DIGIT_W: u32 = 60;
const DIGIT_H: u32 = 120;
const THICKNESS: u32 = 14;
const GAP: i32 = 20;
const COLON_W: i32 = 30;

/// Segment order: a (top), b (top-right), c (bottom-right), d (bottom),
/// e (bottom-left), f (top-left), g (middle).
fn segments_for(digit: u8) -> [bool; 7] {
    match digit {
        0 => [true, true, true, true, true, true, false],
        1 => [false, true, true, false, false, false, false],
        2 => [true, true, false, true, true, false, true],
        3 => [true, true, true, true, false, false, true],
        4 => [false, true, true, false, false, true, true],
        5 => [true, false, true, true, false, true, true],
        6 => [true, false, true, true, true, true, true],
        7 => [true, true, true, false, false, false, false],
        8 => [true, true, true, true, true, true, true],
        9 => [true, true, true, true, false, true, true],
        _ => [false; 7],
    }
}

/// Draws a single seven-segment-style digit.
/// `width` / `height` is the digit's bounding box, `thickness` the bar width
pub fn draw_digit<D>(
    target: &mut D,
    digit: u8,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    thickness: u32,
    color: Rgb666,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb666>,
{
    let segments = segments_for(digit);
    let style = PrimitiveStyle::with_fill(color);
    let half_h = (height - thickness) / 2;

    let bars: [(bool, Rectangle); 7] = [
        // a: top
        (
            segments[0],
            Rectangle::new(Point::new(x, y), Size::new(width, thickness)),
        ),
        // b: top-right
        (
            segments[1],
            Rectangle::new(
                Point::new(x + (width - thickness) as i32, y),
                Size::new(thickness, half_h + thickness),
            ),
        ),
        // c: bottom-right
        (
            segments[2],
            Rectangle::new(
                Point::new(x + (width - thickness) as i32, y + half_h as i32),
                Size::new(thickness, half_h + thickness),
            ),
        ),
        // d: bottom
        (
            segments[3],
            Rectangle::new(
                Point::new(x, y + (height - thickness) as i32),
                Size::new(width, thickness),
            ),
        ),
        // e: bottom-left
        (
            segments[4],
            Rectangle::new(
                Point::new(x, y + half_h as i32),
                Size::new(thickness, half_h + thickness),
            ),
        ),
        // f: top-left
        (
            segments[5],
            Rectangle::new(Point::new(x, y), Size::new(thickness, half_h + thickness)),
        ),
        // g: middle
        (
            segments[6],
            Rectangle::new(
                Point::new(x, y + half_h as i32 - (thickness as i32) / 2),
                Size::new(width, thickness),
            ),
        ),
    ];

    for (active, rect) in bars {
        if active {
            rect.into_styled(style).draw(target)?;
        }
    }

    Ok(())
}

/// Draws "HH:MM" using the seven-segment digits, colon included.
/// `x` / `y` is the top-left corner of the digits
pub fn draw_time<D>(
    target: &mut D,
    hour: u8,
    minute: u8,
    x: i32,
    y: i32,
    color: Rgb666,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb666>,
{
    let digits = [hour / 10, hour % 10, minute / 10, minute % 10];
    let mut cursor_x = x;

    for (i, &d) in digits.iter().enumerate() {
        draw_digit(target, d, cursor_x, y, DIGIT_W, DIGIT_H, THICKNESS, color)?;
        cursor_x += DIGIT_W as i32 + GAP;

        if i == 1 {
            let dot_size = Size::new(THICKNESS, THICKNESS);
            let style = PrimitiveStyle::with_fill(color);
            Rectangle::new(
                Point::new(
                    cursor_x - GAP + (COLON_W - THICKNESS as i32) / 2,
                    y + (DIGIT_H / 3) as i32,
                ),
                dot_size,
            )
            .into_styled(style)
            .draw(target)?;
            Rectangle::new(
                Point::new(
                    cursor_x - GAP + (COLON_W - THICKNESS as i32) / 2,
                    y + (DIGIT_H * 2 / 3) as i32,
                ),
                dot_size,
            )
            .into_styled(style)
            .draw(target)?;
            cursor_x += COLON_W;
        }
    }

    Ok(())
}

/// Total pixel width that `draw_time` occupies — handy for centering.
pub const TIME_WIDTH: u32 = DIGIT_W * 4 + GAP as u32 * 3 + COLON_W as u32;
