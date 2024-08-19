use std::collections::HashMap;

use image::GenericImageView;
use raylib::{RaylibHandle, RaylibThread};

use crate::{tile::Tile, tilemap::Tilemap, Vector2i};

pub struct Level {
    pub tilemap: Tilemap,
}

impl Level {
    pub fn load_from_file(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        tileset: HashMap<u32, (&str, bool, usize)>,
    ) -> Self {
        Self {
            tilemap: load_tilemap(rl, thread, path, tileset),
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
    for (tile_path, solid, _id) in sorted.iter() {
        tiles.push(Tile::new(rl, thread, tile_path, *solid));
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
