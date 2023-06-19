#![allow(dead_code)]
use image::{imageops, ImageBuffer, Rgb, RgbImage};
use rand::Rng;

#[derive(Clone)]
struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Rgb<u8>>>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        let pixels = vec![vec![Rgb([0, 0, 0]); width as usize]; height as usize];
        Image {
            width,
            height,
            pixels,
        }
    }

    fn new_from_file(path: &str) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgb8();

        let width = img.width();
        let height = img.height();

        let mut pixels = vec![vec![Rgb([0, 0, 0]); width as usize]; height as usize];

        for (x, y, pixel) in img.enumerate_pixels() {
            pixels[y as usize][x as usize] = *pixel;
        }

        Ok(Image {
            width,
            height,
            pixels,
        })
    }

    fn generate_gradient_image(&mut self, color1: [u8; 3], color2: [u8; 3]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let r = ((color2[0] as f32 - color1[0] as f32) * (x as f32 / self.width as f32)
                    + color1[0] as f32) as u8;
                let g = ((color2[1] as f32 - color1[1] as f32) * (y as f32 / self.height as f32)
                    + color1[1] as f32) as u8;
                let b = ((color2[2] as f32 - color1[2] as f32)
                    * ((x as f32 + y as f32) / (self.width as f32 + self.height as f32))
                    + color1[2] as f32) as u8;

                self.pixels[y as usize][x as usize] = Rgb([r, g, b]);
            }
        }
    }

    fn resize(&mut self, new_width: u32, new_height: u32) {
        // Create a new vector of pixels with the new dimensions.
        let mut resized_pixels =
            vec![vec![Rgb([0, 0, 0]); new_width as usize]; new_height as usize];
        // Iterate over the new pixels and assign them the appropriate value from the original image.
        for y in 0..new_height {
            for x in 0..new_width {
                // (x as f32 / new_width as f32) divides the current x-coordinate in the resized image by
                // the desired width of the resized image. This ratio represents the relative position of the current
                // x-coordinate in the resized image compared to the desired width. * self.width as f32
                // multiplies the result of the division by the original width of the image. This scaling
                // factor maps the relative position in the resized image to the corresponding position in the original image.
                let original_x = (x as f32 / new_width as f32 * self.width as f32) as u32;
                let original_y = (y as f32 / new_height as f32 * self.height as f32) as u32;
                resized_pixels[y as usize][x as usize] =
                    self.pixels[original_y as usize][original_x as usize];
            }
        }
        self.width = new_width;
        self.height = new_height;
        self.pixels = resized_pixels;
    }

    // This method will result in some lost data, but it works for our purposes.
    fn rotate_90_clockwise(&mut self) {
        let mut rotated_pixels =
            vec![vec![Rgb([0, 0, 0]); self.height as usize]; self.width as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                rotated_pixels[x as usize][(self.height - 1 - y) as usize] =
                    self.pixels[y as usize][x as usize];
            }
        }
        // Swap the width and height. mem::swap is a function that swaps two values.
        std::mem::swap(&mut self.width, &mut self.height);
        self.pixels = rotated_pixels;
    }

    fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let mut cropped_pixels = vec![vec![Rgb([0, 0, 0]); width as usize]; height as usize];
        for y2 in 0..height {
            for x2 in 0..width {
                cropped_pixels[y2 as usize][x2 as usize] =
                    self.pixels[(y + y2) as usize][(x + x2) as usize];
            }
        }
        self.width = width;
        self.height = height;
        self.pixels = cropped_pixels;
    }

    fn add_noise(&mut self, noise_level: u8) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                let noise_r = rng.gen_range(-(2 * noise_level as i16)..=(2 * noise_level as i16));
                let noise_g = rng.gen_range(-(2 * noise_level as i16)..=(2 * noise_level as i16));
                let noise_b = rng.gen_range(-(2 * noise_level as i16)..=(2 * noise_level as i16));

                let pixel = &mut self.pixels[y as usize][x as usize];
                let pixel_r = pixel[0] as i16 + noise_r;
                let pixel_g = pixel[1] as i16 + noise_g;
                let pixel_b = pixel[2] as i16 + noise_b;

                pixel[0] = pixel_r.clamp(0, 255) as u8;
                pixel[1] = pixel_g.clamp(0, 255) as u8;
                pixel[2] = pixel_b.clamp(0, 255) as u8;
            }
        }
    }

    fn display(&self) {
        for row in &self.pixels {
            for pixel in row {
                println!("R: {:3} G: {:3} B: {:3}", pixel[0], pixel[1], pixel[2]);
            }
            println!();
        }
    }

    fn to_rgb_image(&self) -> RgbImage {
        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixels[y as usize][x as usize];
                image.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
            }
        }

        image
    }
}

struct Filter;
impl Filter {
    fn blur(image: &mut Image, sigma: f32) -> RgbImage {
        let blurry_image = imageops::blur(&image.to_rgb_image(), sigma);
        blurry_image
    }

    fn emphasize_blue(image: &mut Image, factor: f32) {
        for y in 0..image.height {
            for x in 0..image.width {
                let pixel = &mut image.pixels[y as usize][x as usize];
                let blue = f32::from(pixel[2]) * factor;
                pixel[2] = blue.min(255.0) as u8;
            }
        }
    }

    fn greenify(image: &mut Image, factor: f32) {
        for y in 0..image.height {
            for x in 0..image.width {
                let pixel = &mut image.pixels[y as usize][x as usize];
                let green = f32::from(pixel[1]) * factor;
                pixel[1] = green.min(255.0) as u8
            }
        }
    }

    fn warm(image: &mut Image, factor: f32) {
        for y in 0..image.height {
            for x in 0..image.width {
                let pixel = &mut image.pixels[y as usize][x as usize];
                let red = f32::from(pixel[0]) * factor;
                let green = f32::from(pixel[1]) * factor;
                pixel[0] = red.min(255.0) as u8;
                pixel[1] = green.min(255.0) as u8;
            }
        }
    }
}

pub fn main() {
    let mut gradient_image = Image::new(8, 8);

    // Generate and save gradient image
    gradient_image.generate_gradient_image([0, 0, 0], [255, 255, 255]);
    let gradient_image_clone = gradient_image.clone().to_rgb_image();
    // gradient_image_clone.display();

    gradient_image_clone
        .save("images/user_generated/original_gradient_image.png")
        .unwrap();

    // Clone and resize the image, then save
    let mut resized_image = gradient_image.clone();
    resized_image.resize(4, 4);
    let resized_image = resized_image.to_rgb_image();
    resized_image
        .save("images/user_generated/resized_gradient_image.png")
        .unwrap();

    // Clone and rotate the image, then save
    let mut rotated_image = gradient_image.clone();
    rotated_image.rotate_90_clockwise();
    let rotated_image = rotated_image.to_rgb_image();
    rotated_image
        .save("images/user_generated/rotated_gradient_image.png")
        .unwrap();

    // Clone the image, add noise to it, and save
    let mut noisy_image = gradient_image.clone();
    noisy_image.add_noise(20);
    let noisy_image = noisy_image.to_rgb_image();
    noisy_image
        .save("images/user_generated/noisy_image.png")
        .unwrap();

    // Ferris Image manipulation.

    // Load ferris image from file
    let img_path = "images/input_image.png";
    let ferris_img = match Image::new_from_file(img_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to open the image at {}: {}", img_path, e);
            return;
        }
    };

    let img_path = "images/sharp.jpg";
    let mut sharp_image = match Image::new_from_file(img_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to open the image at {}: {}", img_path, e);
            return;
        }
    };

    let mut warm_image = sharp_image.clone();
    Filter::warm(&mut warm_image, 1.5);
    warm_image
        .to_rgb_image()
        .save("images/user_generated/warm_image.png")
        .unwrap();

    // Apply blur using the image crate's built-in blur function
    let blurred_image = Filter::blur(&mut sharp_image, 5.0);
    blurred_image
        .save("images/user_generated/blurred_sharp_image.jpg")
        .unwrap();

    // Resize the image
    let mut resized_ferris_img = ferris_img.clone();
    resized_ferris_img.resize(100, 100);
    resized_ferris_img
        .to_rgb_image()
        .save("images/user_generated/resized_ferris.png")
        .unwrap();

    // Rotate the image
    let mut rotated_ferris_img = ferris_img.clone();
    rotated_ferris_img.rotate_90_clockwise();
    rotated_ferris_img
        .to_rgb_image()
        .save("images/user_generated/rotated_ferris.png")
        .unwrap();

    // Crop the image
    let mut cropped_ferris_img = ferris_img.clone();
    cropped_ferris_img.crop(100, 100, 200, 200);
    cropped_ferris_img
        .to_rgb_image()
        .save("images/user_generated/cropped_ferris.png")
        .unwrap();

    // Add noise to the image
    let mut noisy_ferris_img = ferris_img.clone();
    noisy_ferris_img.add_noise(20);
    noisy_ferris_img
        .to_rgb_image()
        .save("images/user_generated/noisy_ferris.png")
        .unwrap();

    // Clone the image, emphasize blue tones, and save
    let mut blue_image = ferris_img.clone();
    Filter::emphasize_blue(&mut blue_image, 5.0);
    blue_image
        .to_rgb_image()
        .save("images/user_generated/blue_ferris.png")
        .unwrap();

    let mut green_image = ferris_img.clone();
    Filter::greenify(&mut green_image, 5.0);
    green_image
        .to_rgb_image()
        .save("images/user_generated/green_ferris.png")
        .unwrap();

}
