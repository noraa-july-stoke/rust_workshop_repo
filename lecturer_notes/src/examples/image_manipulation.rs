#![allow(dead_code)]
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;

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
        let mut resized_pixels =
            vec![vec![Rgb([0, 0, 0]); new_width as usize]; new_height as usize];
        for y in 0..new_height {
            for x in 0..new_width {
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
        std::mem::swap(&mut self.width, &mut self.height);
        self.pixels = rotated_pixels;
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

pub fn main() {
    let mut image = Image::new(8, 8);

    // Generate and save gradient image
    image.generate_gradient_image([0, 0, 0], [255, 255, 255]);
    let gradient_image = image.to_rgb_image();
    gradient_image.save("images/user_generated/original_image.png").unwrap();
    image.display();

    // Resize the image and save
    image.resize(4, 4);
    let resized_image = image.to_rgb_image();
    resized_image.save("images/user_generated/resized_image.png").unwrap();
    image.display();

    // Rotate the image and save
    image.rotate_90_clockwise();
    let rotated_image = image.to_rgb_image();
    rotated_image.save("images/user_generated/rotated_image.png").unwrap();
    image.display();

    // Add noise to the image and save
    image.add_noise(20);
    let noisy_image = image.to_rgb_image();
    noisy_image.save("images/user_generated/noisy_image.png").unwrap();
    image.display();

    let img_path = "images/input_image.png";
    let mut image2 = match Image::new_from_file(img_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to open the image at {}: {}", img_path, e);
            return;
        }
    };

    image2.add_noise(70);
    image2.rotate_90_clockwise();
    // image2.resize(200, 350);

    // Create an RgbImage from image2 and then save it
    let output_image = image2.to_rgb_image();
    output_image.save("images/user_generated/output_image.png").unwrap();

}
