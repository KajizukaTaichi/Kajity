use image::GenericImageView;
use image::Pixel;
use minifb::{Window, WindowOptions};
use std::path::Path;

pub struct Game {
    window: Window,
    buffer: Vec<u32>,
}

impl Game {
    pub fn new(title: String) -> Game {
        let width = 800;
        let height = 600;

        let window =
            Window::new(&title, width, height, WindowOptions::default()).unwrap_or_else(|e| {
                panic!("{}", e);
            });

        Game {
            window,
            buffer: vec![0; width * height],
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            let size = self.window.get_size();
            let _ = self.window.update_with_buffer(&self.buffer, size.0, size.1);
        }
    }

    pub fn image(&mut self, path: &str, point: (u32, u32)) {
        let img = image::open(Path::new(path)).unwrap();
        let (width, height) = img.dimensions();
        let mut buffer = img.to_rgba8().into_raw();

        let x = point.0;
        let y = point.1;

        for j in 0..height {
            for i in 0..width {
                let pixel = img.get_pixel(i, j);
                let rgba = pixel.channels();
                let color = (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | rgba[2] as u32;
                let index = (y + j) * self.window.get_size().0 as u32 + (x + i);
                buffer[index as usize] = color as u8;
            }
        }
        self.buffer = buffer.iter().map(|x|*x as u32).collect();
    }

    pub fn draw(&mut self, color: u32, start: (usize, usize), end: (usize, usize)) {
        let (width, height) = self.window.get_size();
        let mut buffer: Vec<u32> = self.buffer.clone();

        for y in start.1..=end.1 {
            for x in start.0..=end.0 {
                if x < width && y < height {
                    let index = y * width + x;
                    buffer[index] = color;
                }
            }
        }

        self.window
            .update_with_buffer(&self.buffer, width, height)
            .unwrap();
        self.buffer = buffer.clone();
    }
}
