use std::process::exit;

use image::{GenericImageView, Rgba};
use raylib::{RaylibHandle, RaylibThread};

use crate::{tilemap::Tilemap, Vector2i};

pub struct Level {
    pub tilemap: Tilemap,
}

impl Level {
    pub fn load_from_file(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        tileset: Vec<&str>,
    ) -> Self {
        let tilemap = load_tilemap(rl, thread, path, tileset);
        Self { tilemap }
    }
}

fn load_tilemap(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    path: &str,
    tileset: Vec<&str>,
) -> Tilemap {
    let mut textures = Vec::new();
    for tile in tileset.iter() {
        let texture = rl.load_texture(&thread, tile);
        if texture.is_err() {
            println!("COULD NOT LOAD THE TEXTURE: {tile}");
            exit(1);
        }
        textures.push(texture.unwrap());
    }
    let mut tilemap = Tilemap::new(textures);
    let level_image = image::open(path);
    for pixel in level_image.unwrap().pixels() {
        let r = pixel.2 .0[0];
        let g = pixel.2 .0[1];
        let b = pixel.2 .0[2];
        let a = pixel.2 .0[3];

        if a == 255 {
            tilemap.set_tile(Vector2i::new(pixel.0 as i32, pixel.1 as i32), 0);
        }

        println!("{r}, {g}, {b}, {a}");
    }
    return tilemap;
}
