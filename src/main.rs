use std::{collections::HashMap, fmt::Display};

use explosion::Explosion;
use level::Level;
use math::Vector2;
use player::Player;
use raylib::prelude::*;

mod explosion;
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
    placed_bombs: Vec<Vector2i>,
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
        Some(Box::new(|level, _x, _y| {
            level.tilemap.set_tile(Vector2i::new(13, 8), 0)
        })),
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
        Some(Box::new(|level, _x, _y| {
            level.tilemap.set_tile(Vector2i::new(6, 1), 0);
            level.tilemap.set_tile(Vector2i::new(6, 2), 0);
        })),
    ));

    levels.push(Level::load_from_file(
        &mut rl,
        &thread,
        "assets/level4.png",
        tileset.clone(),
        Some(Box::new(|level, x, y| match (x, y) {
            (7, 8) => {
                if level.tilemap.get_tile(&Vector2i::new(5, 5)).unwrap().id() == 1 {
                    level.tilemap.set_tile(Vector2i::new(5, 5), 0);
                    level.tilemap.set_tile(Vector2i::new(12, 1), 1);
                } else {
                    level.tilemap.set_tile(Vector2i::new(5, 5), 1);
                    level.tilemap.set_tile(Vector2i::new(12, 1), 0);
                }
            }
            (13, 8) => {
                level.tilemap.set_tile(Vector2i::new(1, 5), 0);
            }
            (1, 1) => {
                level.tilemap.set_tile(Vector2i::new(11, 1), 0);
            }
            _ => {}
        })),
    ));

    let mut game_state = GameState {
        current_level: levels.remove(0),
        bombs: 0,
        placed_bombs: Vec::new(),
        won: false,
    };

    set_player_pos(&game_state, &mut player);
    let mut started = false;
    let mut explosions: Vec<Explosion> = Vec::new();
    let explosion_textures = vec![
        rl.load_texture(&thread, "assets/explosion1.png").unwrap(),
        rl.load_texture(&thread, "assets/explosion2.png").unwrap(),
        rl.load_texture(&thread, "assets/explosion3.png").unwrap(),
    ];
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if !started {
            render_title_screen(&mut d);
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                started = true;
            }
            continue;
        }

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
                .get_tile(&player.tile_from_center())
                .unwrap()
                .id()
                == 6
                || game_state
                    .current_level
                    .tilemap
                    .get_tile(&player.tile_from_center())
                    .unwrap()
                    .id()
                    == 7
            {
                game_state.current_level.tilemap.set_tile(
                    player.tile_from_center(),
                    if game_state
                        .current_level
                        .tilemap
                        .get_tile(&player.tile_from_center())
                        .unwrap()
                        .id()
                        == 6
                    {
                        7
                    } else {
                        6
                    },
                );
                game_state
                    .current_level
                    .on_lever_flip(player.tile_from_center().x, player.tile_from_center().y);
            } else if game_state
                .current_level
                .tilemap
                .get_tile(&player.tile_from_center())
                .unwrap()
                .id()
                == 8
            {
                game_state
                    .current_level
                    .tilemap
                    .set_tile(player.tile_from_center(), 0);
                game_state.bombs += 1;
            } else if game_state
                .current_level
                .tilemap
                .get_tile(&player.tile_from_center())
                .unwrap()
                .id()
                == 0
            {
                if game_state.bombs >= 1 {
                    game_state
                        .current_level
                        .tilemap
                        .set_tile(player.tile_from_center(), 8);
                    game_state.bombs -= 1;
                    game_state.placed_bombs.push(player.tile_from_center());
                }
            }
        } else if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            detonate_all_bombs(&mut game_state, &mut explosions);
        }

        for explosion in explosions.iter_mut() {
            explosion.render(&mut d, &explosion_textures);
        }

        if game_state
            .current_level
            .tilemap
            .get_tile(&player.tile_from_center())
            .unwrap()
            .id()
            == 4
        {
            // switch levels
            if levels.len() == 0 {
                game_state.won = true;
                continue;
            }
            game_state.current_level = levels.remove(0);
            set_player_pos(&game_state, &mut player);
        }
    }
}

fn render_title_screen(d: &mut RaylibDrawHandle) {
    let text = "PRESS <ENTER> TO START";
    let font_size = 64;
    let width = d.measure_text(&text, font_size);
    d.draw_text(
        text,
        d.get_screen_width() / 2 - width / 2,
        d.get_screen_height() / 2 - (32.0 * d.get_time().sin()) as i32,
        font_size,
        Color::WHITE,
    );
}

fn detonate_all_bombs(game_state: &mut GameState, explosions: &mut Vec<Explosion>) {
    let mut bomb_positions: Vec<Vector2i> = Vec::new();
    for (pos, tile) in game_state.current_level.tilemap.iter() {
        if tile.id() == 8
        /* if its a bomb */
        {
            if game_state.placed_bombs.contains(&pos) {
                bomb_positions.push(pos.clone());
            }
        }
    }
    game_state.placed_bombs.clear();
    for bomb_pos in bomb_positions {
        game_state
            .current_level
            .tilemap
            .set_tile(bomb_pos.clone(), 0);
        explosions.push(Explosion::new(bomb_pos.clone()));
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
