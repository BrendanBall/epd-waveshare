#![deny(warnings)]

use epd_waveshare::{
    color::*,
    epd2in9b_v3::Display2in9b,
    graphics::DisplayRotation,
};
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};

fn main() {
    //println!("Test all the rotations");
    let mut display = Display2in9b::default();

    display.set_rotation(DisplayRotation::Rotate0);
    draw_text(&mut display, "Rotate 0!", 5, 50);
    let buf = display.buffer();
    let width = (128 + 7) / 8;
    for i in 0..width {
        let start = i*296;
        let end = start + 296;
        println!("{:?}", &buf[start..end]);

    }
}

fn draw_text(display: &mut Display2in9b, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(TriColor::White)
        .background_color(TriColor::Black)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}
