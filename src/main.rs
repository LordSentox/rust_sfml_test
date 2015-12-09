//! Example from SFML: Pong

extern crate sfml;
extern crate rand;

mod texture_manager;

use sfml::graphics::{RenderWindow, Color, Font, RenderTarget};
use sfml::window::{VideoMode, ContextSettings, event, window_style, Key};
use sfml::system::Clock;

const GAME_WIDTH: u32 = 800;
const GAME_HEIGHT: u32 = 600;
const FULLSCREEN: bool = false;
const USE_VSYNC: bool = true;

fn main() {
	let style = if FULLSCREEN {
		window_style::FULLSCREEN
	} else {
		window_style::CLOSE
	};

	// Create the window of the application
	let mut window: RenderWindow = RenderWindow::new(
		VideoMode::new_init(GAME_WIDTH, GAME_HEIGHT, 32),
		"SFML Pong",
		style,
		&ContextSettings::default()).unwrap();

	window.set_vertical_sync_enabled(USE_VSYNC);

	let mut clock = Clock::new();

	while window.is_open() {
		println!("Frametime: {}", clock.restart().as_seconds());

		for event in window.events() {
			match event {
				event::Closed => window.close(),
				event::KeyPressed{code, ..} => match code {
					Key::Escape => {
						window.close();
						break;
					},
					_  => {}
				} ,
				_ => {}
			}
		}
		// Clear the window
		window.clear(&Color::new_rgb(50, 200, 50));

		// Display things on screen
		window.display()
	}
}
