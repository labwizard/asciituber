use ::image::Pixel;
use ::image::Rgb;
use ::image::RgbImage;
use ::rand::thread_rng;
use ::rand::Rng;

pub fn image_char(_img: &RgbImage) -> char {
    thread_rng().gen_range('!'..='~')
}

pub fn image_color(img: &RgbImage) -> Rgb<u8> {
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
    Rgb([brightness as u8; 3])
}
