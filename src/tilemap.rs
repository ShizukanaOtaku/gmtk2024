use std::collections::HashMap;

use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

use crate::{Vector2i, SCALE, TILE_SIZE_PIXELS};

pub struct Tilemap {
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
