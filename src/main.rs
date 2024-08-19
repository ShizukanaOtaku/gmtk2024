use std::{collections::HashMap, fmt::Display};

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

struct GameState {
    current_level: Level,
    won: bool,
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
        (0xff00ff, ("assets/lever_on.png", false, 7)),
        (0x00ff00, ("assets/bomb.png", false, 8)),
        (0x146464, ("assets/background.png", false, 9)), // player spawn
    ]);

    levels.push(Level::load_from_file(
        &mut rl,
        &thread,
        "assets/level1.png",
        tileset.clone(),
        Some(Box::new(|&mut _| println!("Level 1 lever flipped"))),
    ));

    levels.push(Level::load_from_file(
        &mut rl,
        &thread,
        "assets/level2.png",
        tileset.clone(),
        None,
    ));

    levels.push(Level::load_from_file(
        &mut rl,
        &thread,
        "assets/level3.png",
        tileset.clone(),
        Some(Box::new(|level: &mut Level| {
            level.tilemap.set_tile(Vector2i::new(6, 1), 0);
            level.tilemap.set_tile(Vector2i::new(6, 2), 0);
        })),
    ));

    let mut game_state = GameState {
        current_level: levels.remove(0),
        won: false,
    };

    set_player_pos(&game_state, &mut player);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if game_state.won {
            let text = "YOU ESCAPED!";
            let width = d.measure_text(&text, 80);
            d.draw_text(
                text,
                d.get_screen_width() / 2 - width / 2,
                d.get_screen_height() / 2 - 50,
                80,
                Color::WHITE,
            );
            let text = "THANKS FOR PLAYING <3";
            let width = d.measure_text(text, 32);
            d.draw_text(
                text,
                d.get_screen_width() / 2 - width / 2,
                d.get_screen_height() / 2 + 50,
                32,
                Color::WHITE,
            );
            continue;
        }

        game_state.current_level.tilemap.render(&mut d);
        player.render(&mut d, &mut game_state);

        if d.is_key_pressed(KeyboardKey::KEY_F)
            && game_state
                .current_level
                .tilemap
                .get_tile(&player.tile_pos_center())
                .unwrap()
                .id()
                == 6
        {
            game_state
                .current_level
                .tilemap
                .set_tile(player.tile_pos_center(), 7);
            game_state.current_level.on_lever_flip();
        }

        if game_state
            .current_level
            .tilemap
            .get_tile(&player.tile_pos_center())
            .unwrap()
            .id()
            == 4
        {
            if levels.len() == 0 {
                println!("W!");
                game_state.won = true;
                continue;
            }
            game_state.current_level = levels.remove(0);
            set_player_pos(&game_state, &mut player);
        }
    }
}

fn set_player_pos(game_state: &GameState, player: &mut Player) {
    for (pos, tile) in game_state.current_level.tilemap.iter() {
        if tile.id() == 9 {
            player.position = Vector2::new(
                (pos.x * TILE_SIZE_PIXELS) as f32,
                (pos.y * TILE_SIZE_PIXELS) as f32,
            );
        }
    }
}
