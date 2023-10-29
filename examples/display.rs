use epd_waveshare::{
    color::*,
    epd2in9b_v3::Display2in9b,
    graphics::DisplayRotation,
};
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Baseline, Text, TextStyleBuilder},
};
use image::{self, ImageBuffer, ImageFormat, Rgb, imageops};

fn main() {
    //println!("Test all the rotations");
    let mut display = Display2in9b::default();
    display.clear(TriColor::White).unwrap();
    display.set_rotation(DisplayRotation::Rotate270);
    draw_text(&mut display, "Rotate 0!", 5, 10);
    let _ = Line::new(Point::new(5, 50), Point::new(291, 50))
    .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 4))
    .draw(&mut display);

    let width = 128u32;
    let height = 296u32;
    save(width, height, display.bw_buffer(), display.chromatic_buffer()).unwrap();
}

fn draw_text(display: &mut Display2in9b, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(TriColor::Black)
        .background_color(TriColor::White)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}

fn to_image(width: u32, height: u32, bw_buffer: &[u8], chromatic_buffer: &[u8]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let data: Vec<u8> = bw_buffer.iter().zip(chromatic_buffer)
    .flat_map(|(black, chromatic)| 
        (0..8u8).rev().flat_map(move |i| 
            to_rgb((black >> i) & 1, (chromatic >> i) & 1).0
        ).collect::<Vec<u8>>()
    )
        .collect();
    
    ImageBuffer::from_vec(width, height, data).unwrap()
}

fn to_rgb(black_bit: u8, red_bit: u8) -> Rgb<u8> {
    let rgb: [u8; 3] = match (black_bit, red_bit) {
        (1, 1) => [255, 255, 255],
        (1, 0) => [255, 0, 0],
        (0, 1) => [0, 0, 0],
        (0, 0) => [255, 0, 0],
        v =>  unreachable!("{:?}", v)
    };
    image::Rgb(rgb)
}

fn save(width: u32, height: u32, bw_buffer: &[u8], chromatic_buffer: &[u8]) -> image::error::ImageResult<()> {
    let img_buffer = to_image(width, height, bw_buffer, chromatic_buffer);
    let image = imageops::rotate90(&img_buffer);
    image.save_with_format("out.png", ImageFormat::Png)?;
    Ok(())
}
