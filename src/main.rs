mod options;

pub use options::*;

use ::ab_glyph::FontRef;
use ::ab_glyph::PxScale;
use ::anyhow::Result;
use ::clap::Parser;
use ::image::GenericImageView;
use ::image::ImageFormat;
use ::image::Pixel;
use ::image::Rgb;
use ::image::RgbImage;
use ::imageproc::drawing::draw_filled_rect_mut;
use ::imageproc::drawing::draw_text_mut;
use ::imageproc::drawing::text_size;
use ::imageproc::rect::Rect;
use ::rand::Rng;
use ::rand::thread_rng;
use ::rscam::Camera;
use ::rscam::Config;
use ::std::fs::File;
use ::std::fs::read;
use ::std::io::Cursor;
use ::std::io::Write;
use ::std::io::stdout;

pub const FPS: u32 = 30;

fn main() -> Result<()> {
    let options = Options::parse();
    let mut camera = Camera::new(&options.input)?;
    camera.start(&Config {
        interval: (1, FPS),
        resolution: (options.width, options.height),
        format: b"MJPG",
        .. Config::default()
    })?;
    let font_bytes = read(options.font_path)?;
    let font = FontRef::try_from_slice(&font_bytes).unwrap();
    let scale = PxScale::from(options.font_size);
    let (char_width, char_height) = text_size(scale, &font, "#");
    let (char_width, char_height) = (char_width as u32, char_height as u32);
    let bg_raw = options.bg_color.trim_start_matches('#');
    if bg_raw.len() != 6 {
        panic!("the background color must be a 6-digit hex number");
    }
    let bg_bytes = u32::from_str_radix(bg_raw, 16)?.to_be_bytes();
    let bg = Rgb([bg_bytes[1], bg_bytes[2], bg_bytes[3]]);
    let fg_raw = options.fg_color.trim_start_matches('#');
    if fg_raw.len() != 6 {
        panic!("the foreground color must be a 6-digit hex number");
    }
    let fg_bytes = u32::from_str_radix(fg_raw, 16)?.to_be_bytes();
    let fg = Rgb([fg_bytes[1], fg_bytes[2], fg_bytes[3]]);
    let alt_fg = if let Some(alt_fg_text) = options.alt_fg_color {
        let alt_fg_raw = alt_fg_text.trim_start_matches('#');
        if alt_fg_raw.len() != 6 {
            panic!("the background color must be a 6-digit hex number");
        }
        let alt_fg_bytes = u32::from_str_radix(alt_fg_raw, 16)?.to_be_bytes();
        Rgb([alt_fg_bytes[1], alt_fg_bytes[2], alt_fg_bytes[3]])
    } else {
        bg
    };
    loop {
        let frame = camera.capture()?;
        let in_image = ::image::load_from_memory(&frame[..])?.to_rgb8();
        let mut out_image = RgbImage::new(in_image.width(), in_image.height());
        draw_filled_rect_mut(
            &mut out_image,
            Rect::at(0, 0).of_size(options.width, options.height),
            bg
        );
        for x in 0..(out_image.width() / char_width) {
            let dx = x * char_width;
            for y in 0..(out_image.height() / char_height) {
                let dy = y * char_height;
                let subimg = in_image.view(dx, dy, char_width, char_height)
                    .to_image();
                let ch = image_char(&subimg);
                let color = image_color(&subimg, alt_fg, fg);
                draw_text_mut(
                    &mut out_image,
                    color,
                    dx as i32,
                    dy as i32,
                    scale,
                    &font,
                    &ch.to_string()
                );
            }
        }
        let mut out_buf = Cursor::new(Vec::new());
        out_image.write_to(&mut out_buf, ImageFormat::Jpeg)?;
        if options.output == "-" {
            stdout().write(out_buf.get_ref())?;
        } else {
            let mut out_file = File::create(&options.output)?;
            out_file.write(out_buf.get_ref())?;
        }
        if options.single_frame {
            break;
        } else {
            println!();
        }
    }
    Ok(())
}

/// Returns the character associated with a subsection of `img`.
pub fn image_char(_img: &RgbImage) -> char {
    thread_rng().gen_range('!'..='~')
}

/// Returns the color associated with a subsection of `img`.
pub fn image_brightness(img: &RgbImage) -> f32 {
    let pixel_count = img.width() * img.height();
    let mut total_rgb = [0; 3];
    for rgb in img.pixels().map(|p| p.to_rgb().0) {
        for i in 0..3 {
            total_rgb[i] += rgb[i] as u32;
        }
    }
    let mean_rgb = total_rgb.map(|x| x / pixel_count);
    let mut deviation = 0;
    for rgb in img.pixels().map(|p| p.to_rgb().0) {
        for i in 0..3 {
            deviation += mean_rgb[i].abs_diff(rgb[i] as u32);
        }
    }
    ((4 * deviation / pixel_count) as f32).powf(2.5) / 256.0f32.powf(2.5)
}

pub fn image_color(
    img: &RgbImage,
    color1: Rgb<u8>,
    color2: Rgb<u8>
) -> Rgb<u8> {
    let b = image_brightness(img);
    Rgb([0, 1, 2].map(|i| {
        (color1.0[i] as f32 * (1.0 - b) + color2.0[i] as f32 * b) as u8
    }))
}
