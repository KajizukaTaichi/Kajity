use minifb::{Window, WindowOptions};

pub struct Game {
    window: Window,
    buffer: Vec<u32>
}

impl Game {
    pub fn new(title: String) -> Game {
        let width = 800;
        let height = 600;

        let window = Window::new(
            &title,
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Game {
            window,
            buffer: vec![0; width * height]
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            let size = self.window.get_size();
            let _ = self.window.update_with_buffer(&self.buffer, size.0, size.1);
        }
    }    

    pub fn draw(&mut self, color: u32, start: (usize, usize), end:(usize, usize)) {
        let (width ,height) = self.window.get_size();
        let mut buffer: Vec<u32> = self.buffer.clone();
        
        for y in start.1..=end.1 {
            for x in start.0..=end.0 {
                if x < width && y < height {
                    let index = y * width + x;
                    buffer[index] = color;
                }
            }
        }
        
        self.window.update_with_buffer(&self.buffer, width, height).unwrap();
        self.buffer = buffer.clone();
    }
}
