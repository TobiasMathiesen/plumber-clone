extern crate ggez;

mod player;
mod object;
mod bbox;
mod sprite;
mod state;
mod input;
mod enemy;

use bbox::BBox;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use object::Object;
use state::MainState;
use std::env;
use std::path;

const SCREEN_WIDTH: u32 = 512;
const SCREEN_HEIGHT: u32 = 512;

fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);
    let ctx = &mut ggez::ContextBuilder::new("mario", "skuzzi")
        .window_setup(ggez::conf::WindowSetup::default().title("Mario"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .expect("Failed to build ggez context");

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
