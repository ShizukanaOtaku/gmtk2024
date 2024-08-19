use std::{collections::HashMap, fmt::Display, process::exit};

use level::Level;
use math::Vector2;
use player::Player;
use raylib::prelude::*;

mod level;
mod player;
mod tile;
mod tilemap;
mod utils;

static SCALE: i32 = 4;
static TILEMAP_WIDTH: i32 = 16;
static TILEMAP_HEIGHT: i32 = 10;
static TILE_SIZE_PIXELS: i32 = 16 * SCALE;

#[derive(Eq, PartialEq, Hash)]
pub struct Vector2i {
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

struct GameState<'a> {
    current_level: &'a Level,
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
    let bg_color = Color::from_hex("323232").unwrap();

    let player_texture = rl.load_texture(&thread, "assets/player.png").unwrap();
    let mut player = Player::new(Vector2::zero(), player_texture);

    let mut levels: Vec<Level> = Vec::new();
    let tileset = HashMap::from([
        (0xFFFFFF, ("assets/background.png", false, 0)),
        (0x0, ("assets/wall.png", true, 1)),
        (0x143c96, ("assets/vent.png", false, 2)),
        (0x14a064, ("assets/dead_robot.png", false, 3)),
        (0xff0000, ("assets/exit.png", false, 4)),
        (0x5a5a5a, ("assets/weak_wall.png", true, 5)),
        (0xff00f0, ("assets/lever_off.png", false, 6)),
        (0x00ff00, ("assets/bomb.png", false, 7)),
        (0x146464, ("assets/background.png", false, 8)), // player spawn
    ]);

    levels.push(Level::load_from_file(
        &mut rl,
        &thread,
        "assets/level1.png",
        tileset.clone(),
    ));

    levels.push(Level::load_from_file(
        &mut rl,
        &thread,
        "assets/level2.png",
        tileset.clone(),
    ));

    let mut level_index = 0;
    let mut game_state = GameState {
        current_level: levels.get(level_index).unwrap(),
    };

    set_player_pos(&game_state, &mut player);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);

        game_state.current_level.tilemap.render(&mut d);
        player.render(&mut d, &mut game_state);

        if game_state
            .current_level
            .tilemap
            .get_tile(&player.tile_pos_center())
            .unwrap()
            .id()
            == 4
        {
            level_index += 1;
            if level_index >= levels.len() {
                println!("W!");
                exit(0);
            }
            game_state.current_level = levels.get(level_index).unwrap();
            set_player_pos(&game_state, &mut player);
        }
    }
}

fn set_player_pos(game_state: &GameState, player: &mut Player) {
    for (pos, tile) in game_state.current_level.tilemap.iter() {
        if tile.id() == 8 {
            player.position = Vector2::new(
                (pos.x * TILE_SIZE_PIXELS) as f32,
                (pos.y * TILE_SIZE_PIXELS) as f32,
            );
        }
    }
}
