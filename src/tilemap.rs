use std::collections::HashMap;

use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{tile::Tile, Vector2i, SCALE, TILE_SIZE_PIXELS};

pub struct Tilemap {
    tiles: HashMap<Vector2i, usize>,
    tileset: Vec<Tile>,
}

impl Tilemap {
    pub fn new(tileset: Vec<Tile>) -> Self {
        Self {
            tiles: HashMap::new(),
            tileset,
        }
    }

    pub fn set_tile(&mut self, pos: Vector2i, tile: usize) {
        self.tiles.insert(pos, tile);
    }

    pub fn get_tile(&self, pos: &Vector2i) -> Option<&Tile> {
        let tile_id = self.tiles.get(&pos);
        if tile_id.is_none() {
            return None;
        }
        self.tileset.get(*tile_id.unwrap())
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        for (pos, _id) in self.tiles.iter() {
            d.draw_texture_ex(
                &self
                    .get_tile(&pos)
                    .unwrap_or(self.tileset.get(0).unwrap())
                    .texture(),
                Vector2::new(pos.x as f32, pos.y as f32).scale_by((TILE_SIZE_PIXELS) as f32),
                0.0,
                SCALE as f32,
                Color::WHITE,
            )
        }
    }

    pub fn collides(&self, hitbox: &Rectangle) -> bool {
        for tile in self.tiles.keys() {
            {
                let tile = self.get_tile(tile);
                if tile.is_some() && !tile.unwrap().solid() {
                    continue;
                }
            }
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

use std::collections::hash_map::Iter;

pub struct TilemapIterator<'a> {
    map_iter: Iter<'a, Vector2i, usize>,
    tileset: &'a Vec<Tile>,
}

impl<'a> Iterator for TilemapIterator<'a> {
    type Item = (&'a Vector2i, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, &tile_id)) = self.map_iter.next() {
            if let Some(tile) = self.tileset.get(tile_id) {
                return Some((pos, tile));
            }
        }
        None
    }
}

impl Tilemap {
    pub fn iter(&self) -> TilemapIterator {
        TilemapIterator {
            map_iter: self.tiles.iter(),
            tileset: &self.tileset,
        }
    }

    pub fn iter_mut(&mut self) -> TilemapIterator {
        TilemapIterator {
            map_iter: self.tiles.iter(),
            tileset: &self.tileset,
        }
    }
}
