use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub struct Tile {
    texture: Texture2D,
    solid: bool,
}

impl Tile {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str, solid: bool) -> Self {
        Self {
            texture: rl.load_texture(thread, path).unwrap(),
            solid,
        }
    }

    pub fn texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn solid(&self) -> bool {
        self.solid
    }
}
