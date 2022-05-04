use amethyst::{
    core::{math::Vector2, Transform},
    ecs::{Component, DenseVecStorage},
};

pub struct Rigidbody2D {
    transform: Transform,
}

impl Rigidbody2D {
    pub fn new() -> Self {
        Rigidbody2D {
            transform: Transform::default(),
        }
    }

    pub fn get_position(&self) -> Vector2<f32> {
        Vector2::new(
            self.transform.translation().x,
            self.transform.translation().y,
        )
    }

    pub fn get_rotation(&self) -> f32 {
        self.transform.rotation().angle()
    }
}

impl Component for Rigidbody2D {
    type Storage = DenseVecStorage<Self>;
}
