mod image_to_ascii;
mod options;

pub use image_to_ascii::*;
pub use options::*;

use ::anyhow::Result;
use ::clap::Parser;
use ::image::GenericImageView;
use ::image::ImageOutputFormat;
use ::image::RgbImage;
use ::imageproc::drawing::draw_text_mut;
use ::imageproc::drawing::text_size;
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
    let mut camera = Camera::new(&options.input).unwrap();
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
    loop {
        let frame = camera.capture()?;
        let in_image = ::image::load_from_memory(&frame[..])?.to_rgb8();
        let mut out_image = RgbImage::new(in_image.width(), in_image.height());
        for x in 0..(out_image.width() / char_width) {
            let dx = x * char_width;
            for y in 0..(out_image.height() / char_height) {
                let dy = y * char_height;
                let subimg = in_image.view(dx, dy, char_width, char_height)
                    .to_image();
                let ch = image_char(&subimg);
                let fg = image_color(&subimg);
                draw_text_mut(
                    &mut out_image,
                    fg,
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
