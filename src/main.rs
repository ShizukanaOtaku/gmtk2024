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

#[derive(Eq, PartialEq, Hash, Clone)]
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
    bombs: i32,
    can_detonate: bool,
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
        bombs: 0,
        can_detonate: false,
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

        if d.is_key_pressed(KeyboardKey::KEY_F) {
            if game_state
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
            } else if game_state
                .current_level
                .tilemap
                .get_tile(&player.tile_pos_center())
                .unwrap()
                .id()
                == 8
            {
                game_state
                    .current_level
                    .tilemap
                    .set_tile(player.tile_pos_center(), 0);
                game_state.bombs += 1;
                game_state.can_detonate = true;
            } else if game_state
                .current_level
                .tilemap
                .get_tile(&player.tile_pos_center())
                .unwrap()
                .id()
                == 0
            {
                if game_state.bombs >= 1 {
                    game_state
                        .current_level
                        .tilemap
                        .set_tile(player.tile_pos_center(), 8);
                    game_state.bombs -= 1;
                }
            }
        } else if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            if game_state.can_detonate {
                detonate_all_bombs(&mut game_state);
            }
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
                game_state.won = true;
                continue;
            }
            game_state.current_level = levels.remove(0);
            set_player_pos(&game_state, &mut player);
        }
    }
}

fn detonate_all_bombs(game_state: &mut GameState) {
    let mut bomb_positions: Vec<Vector2i> = Vec::new();
    for (pos, tile) in game_state.current_level.tilemap.iter() {
        if tile.id() == 8 {
            bomb_positions.push(pos.clone());
        }
    }
    for bomb_pos in bomb_positions {
        game_state
            .current_level
            .tilemap
            .set_tile(bomb_pos.clone(), 0);
        for x in (bomb_pos.x - 1)..=(bomb_pos.x + 1) {
            for y in (bomb_pos.y - 1)..=(bomb_pos.y + 1) {
                if game_state
                    .current_level
                    .tilemap
                    .get_tile(&Vector2i::new(x, y))
                    .unwrap()
                    .id()
                    == 5
                {
                    game_state
                        .current_level
                        .tilemap
                        .set_tile(Vector2i::new(x, y), 0);
                }
            }
        }
    }
    game_state.can_detonate = false;
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
