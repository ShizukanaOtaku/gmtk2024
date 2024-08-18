use std::collections::HashMap;
use std::fmt::Display;

use math::Rectangle;
use player::Player;
use raylib::prelude::*;
use raylib::texture::Texture2D;

mod player;
mod utils;

static SCALE: i32 = 8;
static TILEMAP_WIDTH: i32 = 16;
static TILEMAP_HEIGHT: i32 = 10;
static TILE_SIZE_PIXELS: i32 = 8 * SCALE;

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

struct GameObject {
    pub position: Vector2i,
    texture: Texture2D,
}

impl GameObject {
    pub fn new(position: Vector2i, texture: Texture2D) -> Self {
        Self { position, texture }
    }

    pub fn render(&mut self, display: &mut RaylibDrawHandle, gamestate: &mut GameState) {
        display.draw_texture_ex(
            &self.texture,
            Vector2::new(
                (self.position.x * SCALE) as f32,
                (self.position.y * SCALE) as f32,
            ),
            0.0,
            SCALE as f32,
            Color::WHITE,
        );
    }

    pub fn hitbox(&self) -> Rectangle {
        Rectangle {
            x: (self.position.x * SCALE) as f32,
            y: (self.position.y * SCALE) as f32,
            width: TILE_SIZE_PIXELS as f32,
            height: TILE_SIZE_PIXELS as f32,
        }
    }

    pub fn collides(&self, other: &GameObject) -> bool {
        return self.hitbox().check_collision_recs(&other.hitbox());
    }
}

struct Tilemap {
    tiles: HashMap<Vector2i, usize>,
    textures: Vec<Texture2D>,
}

impl Tilemap {
    pub fn new(textures: Vec<Texture2D>) -> Self {
        Self {
            tiles: HashMap::new(),
            textures,
        }
    }

    pub fn set_tile(&mut self, pos: Vector2i, tile: usize) {
        self.tiles.insert(pos, tile);
    }

    pub fn get_tile(&self, pos: Vector2i) -> Option<&usize> {
        self.tiles.get(&pos)
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        for (pos, tile) in self.tiles.iter() {
            d.draw_texture_ex(
                &self.textures.get(*tile).unwrap(),
                Vector2::new(pos.x as f32, pos.y as f32).scale_by((TILE_SIZE_PIXELS) as f32),
                0.0,
                SCALE as f32,
                Color::WHITE,
            )
        }
    }

    pub fn collides(&self, hitbox: &Rectangle) -> bool {
        for tile in self.tiles.keys() {
            let r = Rectangle {
                x: (tile.x * TILE_SIZE_PIXELS) as f32,
                y: (tile.y * TILE_SIZE_PIXELS) as f32,
                width: (TILE_SIZE_PIXELS) as f32,
                height: (TILE_SIZE_PIXELS) as f32,
            };
            if r.check_collision_recs(hitbox) {
                return true;
            }
        }
        return false;
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
        Vector2::new(0.0, TILE_SIZE_PIXELS as f32 * 3.0),
        player_texture,
    );
    let bg_color = Color::from_hex("323232").unwrap();
    let mut gamestate = GameState {
        tilemap: Tilemap::new(vec![rl.load_texture(&thread, "assets/test.png").unwrap()]),
    };
    for x in 0..=10 {
        gamestate.tilemap.set_tile(Vector2i::new(x, 6), 0);
    }

    for x in 0..=10 {
        gamestate.tilemap.set_tile(Vector2i::new(x, 0), 0);
    }

    gamestate.tilemap.set_tile(Vector2i::new(6, 3), 0);
    gamestate.tilemap.set_tile(Vector2i::new(6, 2), 0);
    gamestate.tilemap.set_tile(Vector2i::new(3, 5), 0);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);

        player.render(&mut d, &mut gamestate);
        gamestate.tilemap.render(&mut d);

        d.draw_text(d.get_fps().to_string().as_str(), 0, 0, 24, Color::RED);
    }
}
