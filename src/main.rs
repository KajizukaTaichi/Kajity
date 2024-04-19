mod graphics;
use graphics::Game;

fn main() {
    let mut game = Game::new("test".to_string());
    game.draw(0x0000FF, (10, 10), (50, 50));
    game.draw(0x00BBAA, (20, 80), (50, 90));
    game.image("C:/Users/admin/OneDrive/画像/logic.png", (1,10));
    game.run();
}
