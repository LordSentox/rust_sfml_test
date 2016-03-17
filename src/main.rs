extern crate sfml;
extern crate rand;

use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite};
use sfml::window::{Key, VideoMode, event, window_style, ContextSettings};

mod texture_manager;
use texture_manager::TextureManager;

fn main() {
    // コンスタントを作る。
    let game_width: u32 = 440;
    let game_height: u32 = 440;

    // アプリのウインドーを作る。
    let mut window = RenderWindow::new(VideoMode::new_init(game_width, game_height, 32),
                                       "SFML Pong",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
                         .unwrap();
    window.set_vertical_sync_enabled(true);

    // 画をロードしてる
    let mut texture_manager = TextureManager::new();

    let sprite = Sprite::new_with_texture(texture_manager.find("resources/frank.jpeg").unwrap()).unwrap();

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => return,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Clear the window
        window.clear(&Color::new_rgb(50, 200, 50));

        window.draw(&sprite);

        // Display things on screen
        window.display()
    }

}
