use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Copy)]
pub struct Animation {
    pub frames: i32,
    pub frame_duration: u64,
    pub first_sprite_index: usize,
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}
