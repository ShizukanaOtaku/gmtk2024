use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub struct Tile {
    texture: Texture2D,
    solid: bool,
    id: usize,
}

impl Tile {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        solid: bool,
        id: usize,
    ) -> Self {
        Self {
            texture: rl.load_texture(thread, path).unwrap(),
            solid,
            id,
        }
    }

    pub fn texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn solid(&self) -> bool {
        self.solid
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
