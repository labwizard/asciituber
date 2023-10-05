mod image_to_ascii;
mod options;

pub use image_to_ascii::*;
pub use options::*;

use ::anyhow::Result;
use ::clap::Parser;
use ::image::GenericImageView;
use ::image::ImageOutputFormat;
use ::image::Rgb;
use ::image::RgbImage;
use ::imageproc::drawing::draw_text_mut;
use ::imageproc::drawing::draw_filled_rect_mut;
use ::imageproc::drawing::text_size;
use ::imageproc::rect::Rect;
use ::rscam::Camera;
use ::rscam::Config;
use ::rusttype::Font;
use ::rusttype::Scale;
use ::std::fs::File;
use ::std::fs::read;
use ::std::io::Cursor;
use ::std::io::Seek;
use ::std::io::SeekFrom;
use ::std::io::Write;
use ::std::io::stdout;

fn main() -> Result<()> {
    let options = Options::parse();
    let mut camera = Camera::new(&options.input)?;
    camera.start(&Config {
        interval: (1, 30),
        resolution: (options.width, options.height),
        format: b"MJPG",
        .. Config::default()
    })?;
    let mut out_file = if options.output == "-" {
        None
    } else {
        Some(File::create(options.output)?)
    };
    let font_bytes = read(options.font_path)?;
    let font = Font::try_from_bytes(&font_bytes).unwrap();
    let scale = Scale::uniform(options.font_size);
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
        out_image.write_to(&mut out_buf, ImageOutputFormat::Png)?;
        if let Some(out_file) = &mut out_file {
            out_file.seek(SeekFrom::Start(0))?;
            out_file.write(out_buf.get_ref())?;
        } else {
            stdout().write(out_buf.get_ref())?;
        }
        if options.single_frame {
            break Ok(());
        }
    }
}
