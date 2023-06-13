use image::{Rgb, RgbImage};

fn main() {
    let (width, height) = (100, 100);

    let mut image = RgbImage::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = (x as f32 / (width - 1) as f32 * 255.0) as u8;
        let g = (y as f32 / (height - 1) as f32 * 255.0) as u8;

        *pixel = Rgb([r, g, 0]);
    }

    image.save("image.png").unwrap();
}
