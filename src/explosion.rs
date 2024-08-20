use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

use crate::{Vector2i, TILE_SIZE_PIXELS};

pub struct Explosion {
    position: Vector2i,
    timer: i32,
}

impl Explosion {
    pub fn new(position: Vector2i) -> Self {
        Self { position, timer: 0 }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, textures: &Vec<Texture2D>) {
        if self.finished() {
            return;
        }
        let index = (self.timer as f32 / (20.0 / textures.len() as f32)).floor() as i32;
        let texture = &textures.get(index as usize).unwrap();
        let scale = 5.0;
        d.draw_texture_ex(
            texture,
            Vector2::new(
                (self.position.x as f32 + 0.5) * TILE_SIZE_PIXELS as f32
                    - texture.width as f32 * scale / 2.0,
                (self.position.y as f32 + 0.5) * TILE_SIZE_PIXELS as f32
                    - texture.height as f32 * scale / 2.0,
            ),
            0.0,
            scale,
            Color::WHITE,
        );
        self.timer += 1;
    }

    pub fn finished(&self) -> bool {
        self.timer > 15
    }
}
