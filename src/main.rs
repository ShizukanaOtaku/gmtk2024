use std::fmt::Display;

use level::Level;
use player::Player;
use raylib::prelude::*;
use tilemap::Tilemap;

mod level;
mod player;
mod tilemap;
mod utils;
mod tile;

static SCALE: i32 = 4;
static TILEMAP_WIDTH: i32 = 16;
static TILEMAP_HEIGHT: i32 = 10;
static TILE_SIZE_PIXELS: i32 = 16 * SCALE;

#[derive(Eq, PartialEq, Hash)]
struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Display for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{{}, {}}}", self.x, self.y))
    }
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct GameState {
    tilemap: Tilemap,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(
            TILEMAP_WIDTH * TILE_SIZE_PIXELS,
            TILEMAP_HEIGHT * TILE_SIZE_PIXELS,
        )
        .title("Hello, World")
        .build();

    rl.set_exit_key(None);
    rl.set_target_fps(60);

    let player_texture = rl.load_texture(&thread, "assets/player.png").unwrap();
    let mut player = Player::new(
        Vector2::new((TILE_SIZE_PIXELS * 3) as f32, (TILE_SIZE_PIXELS * 3) as f32),
        player_texture,
    );

    let level = Level::load_from_file(
        &mut rl,
        &thread,
        "assets/level1.png",
        vec!["assets/background.png"],
    );
    let bg_color = Color::from_hex("323232").unwrap();
    let mut gamestate = GameState {
        tilemap: level.tilemap,
    };

    gamestate.tilemap.set_tile(Vector2i::new(3, 4), 0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);

        gamestate.tilemap.render(&mut d);
        player.render(&mut d, &mut gamestate);
    }
}
