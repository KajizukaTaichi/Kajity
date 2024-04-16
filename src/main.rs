mod graphics;
use graphics::Game;

fn main() {
    let mut game = Game::new("test".to_string());
    game.draw(0x0000FF, (0,0), (500,500));
    game.draw(0x0000AA, (0,0), (200,800));
    game.run();
    
}
