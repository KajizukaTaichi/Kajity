mod graphics;
use graphics::Game;

fn main() {
    let mut game = Game::new("test".to_string());
    game.draw(|buffer, width, height| {
        for y in 0..height {
            buffer[y * width] = 0x0000FF; // Blue
        }
    });
    game.run();
    
}
