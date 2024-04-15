use minifb::{Key, Window, WindowOptions};

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
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // ウィンドウを更新
            self.window.update_with_buffer(&self.buffer as &[u32], self.window.get_size().0, self.window.get_size().1).unwrap();
    
            // 60FPSで待機
            std::thread::sleep(std::time::Duration::from_secs(1) / 60);
        }
    }    

    pub fn draw<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut [u32], usize, usize),
    {
        let size = self.window.get_size();
        let mut buffer: Vec<u32> = vec![0; size.0 * size.1];
        f(&mut buffer, size.0, size.1);
        self.buffer = buffer.clone();
    }
}
