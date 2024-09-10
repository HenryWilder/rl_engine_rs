#![deny(missing_docs)]
#![allow(dead_code)]

//! A Raylib game engine written in Rust

use raylib::prelude::*;
mod game_handle;
pub mod draw;
pub mod collide;
pub mod asset;
pub mod prelude;

use game_handle::GameHandle;

/// A struct for interacting with the game
///
/// Example:
/// ```
/// struct Player {}
///
/// impl Draw for Player {
///     fn draw(&self, d: &mut impl RaylibDraw) {
///
///     }
/// }
///
/// # use rl_engine_rs::prelude::*;
/// fn main() {
///     Game::new(640, 480, "My Game")
///         .load_static()
///         .run();
/// }
/// ```
pub struct Game {
    handle: GameHandle,
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl Game {
    /// Create a new game window
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        let (rl, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();

        Self { handle: GameHandle::new(), rl, thread }
    }

    /// Run the game until it closes
    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            self.handle.tick();
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::BLACK);
            self.handle.draw(&mut d);
        }
    }
}
