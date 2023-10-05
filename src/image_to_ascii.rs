use ::image::Pixel;
use ::image::Rgb;
use ::image::RgbImage;
use ::rand::thread_rng;
use ::rand::Rng;

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
    (4 * deviation / pixel_count).pow(2) as f32 / 65536.0
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
