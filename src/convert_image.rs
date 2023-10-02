use ::crossterm::style::Color;
use ::image::Pixel;
use ::image::RgbImage;
use ::rand::thread_rng;
use ::rand::Rng;

pub fn image_char(_img: &RgbImage) -> char {
    thread_rng().gen_range('!'..='~')
}

pub fn image_color(img: &RgbImage) -> Color {
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
    let brightness = ((6 * deviation / pixel_count).pow(2) / 256).min(255);
    Color::Rgb {
        r: brightness as u8,
        g: brightness as u8,
        b: brightness as u8
    }
}
