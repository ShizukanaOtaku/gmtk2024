use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

use crate::{GameState, Tilemap, SCALE, TILE_SIZE_PIXELS};

pub struct Player {
    position: Vector2, // position in pixels on the screen
    texture: Texture2D,
    scale: f32,
    pub velocity: Vector2,
    pub on_ground: bool,
}

impl Player {
    pub fn new(position: Vector2, texture: Texture2D) -> Self {
        Self {
            position,
            scale: 1.0,
            texture,
            on_ground: false,
            velocity: Vector2::zero(),
        }
    }

    pub fn render(&mut self, display: &mut RaylibDrawHandle, game_state: &mut GameState) {
        self.velocity.y += 2.2 * ((self.scale + 1.0) / 2.0); // Gravity

        if self.position.y
            >= display.get_screen_height() as f32 - TILE_SIZE_PIXELS as f32 * self.scale
        {
            self.on_ground = true;
        }

        self.handle_input(display, &game_state.tilemap);
        self.move_vertical(game_state);
        self.move_horizontal(game_state);

        self.velocity *= 0.8; // apply drag

        self.clamp_position(display); // don't fall out of the screen
        display.draw_texture_ex(
            &self.texture,
            Vector2::new(self.position.x as f32, self.position.y as f32),
            0.0,
            self.scale * SCALE as f32,
            Color::WHITE,
        );
    }

    fn move_horizontal(&mut self, game_state: &mut GameState) {
        self.position.x += self.velocity.x;
        let mut i = 0;
        while game_state.tilemap.collides(&self.hitbox()) {
            self.position.x -= self.velocity.x;
            if i >= 10 {
                break;
            }
            i += 1;
        }
    }

    fn move_vertical(&mut self, game_state: &mut GameState) {
        self.position.y += self.velocity.y;
        self.on_ground = false;
        let mut i = 0;
        while game_state.tilemap.collides(&self.hitbox()) {
            self.position.y -= self.velocity.y;
            self.on_ground = self.velocity.y >= 0.0;
            self.velocity.y = 0.0;
            if i >= 10 {
                break;
            }
            i += 1;
        }
    }

    fn handle_input(&mut self, display: &mut RaylibDrawHandle, tilemap: &Tilemap) {
        let speed = 1.5;
        if display.is_key_down(raylib::ffi::KeyboardKey::KEY_D) {
            self.velocity.x += speed;
        }
        if display.is_key_down(raylib::ffi::KeyboardKey::KEY_A) {
            self.velocity.x -= speed;
        }
        if self.on_ground && display.is_key_down(raylib::ffi::KeyboardKey::KEY_SPACE) {
            self.velocity.y = -60.0 * (self.scale / 2.0); // jump
        }

        let size_change = 0.05;
        if self.scale < 3.0 && display.is_key_down(raylib::ffi::KeyboardKey::KEY_UP) {
            self.scale += size_change * 2.0;
            self.position.y -= size_change * 2.0 * TILE_SIZE_PIXELS as f32;
            if tilemap.collides(&self.hitbox()) {
                self.scale -= size_change;
                self.position.y += size_change * TILE_SIZE_PIXELS as f32;
            }
            self.scale -= size_change;
        }
        if self.scale > 1.0 && display.is_key_down(raylib::ffi::KeyboardKey::KEY_DOWN) {
            self.scale -= size_change;
        }
    }

    pub fn hitbox(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: TILE_SIZE_PIXELS as f32 * self.scale,
            height: TILE_SIZE_PIXELS as f32 * self.scale,
        }
    }

    fn clamp_position(&mut self, display: &mut RaylibDrawHandle) {
        self.position.x = self.position.x.clamp(
            0.0,
            display.get_screen_width() as f32 - TILE_SIZE_PIXELS as f32 * self.scale,
        );
        self.position.y = self.position.y.clamp(
            0.0,
            display.get_screen_height() as f32 - TILE_SIZE_PIXELS as f32 * self.scale,
        );
    }
}
