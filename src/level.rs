use std::collections::HashMap;

use image::GenericImageView;
use raylib::{RaylibHandle, RaylibThread};

use crate::{tile::Tile, tilemap::Tilemap, Vector2i};

pub struct Level {
    pub tilemap: Tilemap,
    lever_hook: Option<Box<dyn FnMut(&mut Self)>>,
}

impl Level {
    pub fn load_from_file(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        tileset: HashMap<u32, (&str, bool, usize)>,
        lever_trigger: Option<Box<dyn FnMut(&mut Self)>>,
    ) -> Self {
        Self {
            tilemap: load_tilemap(rl, thread, path, tileset),
            lever_hook: lever_trigger,
        }
    }

    pub fn on_lever_flip(&mut self) {
        if let Some(mut hook) = self.lever_hook.take() {
            hook(self);
            self.lever_hook = Some(hook);
        }
    }
}

fn load_tilemap(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    path: &str,
    tileset: HashMap<u32, (&str, bool, usize)>,
) -> Tilemap {
    let mut tiles = Vec::new();
    let mut sorted: Vec<_> = tileset.values().clone().collect();
    sorted.sort_by_key(|k| k.2);
    for (tile_path, solid, id) in sorted.iter() {
        tiles.push(Tile::new(rl, thread, tile_path, *solid, *id));
    }

    let mut tilemap = Tilemap::new(tiles);
    let level_image = image::open(path);
    for pixel in level_image.unwrap().pixels() {
        let r = pixel.2 .0[0] as u32;
        let g = pixel.2 .0[1] as u32;
        let b = pixel.2 .0[2] as u32;
        let a = pixel.2 .0[3];

        let code = r << 16 | g << 8 | b;

        let tile_id = tileset.get(&code);
        if tile_id.is_some() && a == 255 {
            let tile_data = tile_id.unwrap();
            tilemap.set_tile(
                Vector2i::new(pixel.0 as i32, pixel.1 as i32),
                tile_data.2, // the id
            );
        } else {
            println!("Unknown color code: {code}");
            tilemap.set_tile(Vector2i::new(pixel.0 as i32, pixel.1 as i32), 0);
        }
    }
    return tilemap;
}
