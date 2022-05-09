use std::ops::Mul;

use amethyst::core::math::{Vector, Vector2};
use amethyst::ecs::{Component, DenseVecStorage};

use super::Rigidbody2D;

pub struct Collider2D {
    pub offset: Vector2<f32>,
}

impl Collider2D {
    pub fn new(offset: Vector2<f32>) -> Self {
        Collider2D { offset: offset }
    }
}

impl Component for Collider2D {
    type Storage = DenseVecStorage<Self>;
}

pub struct Circle {
    radius: f32,
    body: Rigidbody2D,
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle {
            radius: radius,
            body: Rigidbody2D::new(),
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_center(&self) -> Vector2<f32> {
        self.body.get_position()
    }
}

pub struct CircleCollider2D {
    collider2d: Collider2D,
    pub radius: f32,
}

impl CircleCollider2D {
    pub fn new(offset: Vector2<f32>, radius: f32) -> Self {
        CircleCollider2D {
            collider2d: Collider2D::new(offset),
            radius: radius,
        }
    }

    pub fn get_offset(&self) -> Vector2<f32> {
        self.collider2d.offset
    }
}

impl Component for CircleCollider2D {
    type Storage = DenseVecStorage<Self>;
}

pub struct BoxCollider2D {
    size: Vector2<f32>,
    half_size: Vector2<f32>,
    rigidbody2d: Rigidbody2D,
}

impl BoxCollider2D {
    pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        let size = Vector2::new(max.x - min.x, max.y - min.y);
        BoxCollider2D {
            size: size,
            half_size: size.mul(0.5),
            rigidbody2d: Rigidbody2D::new(),
        }
    }

    pub fn get_min(&self) -> Vector2<f32> {
        Vector2::new(
            self.rigidbody2d.get_position().x - self.half_size.x,
            self.rigidbody2d.get_position().y - self.half_size.y,
        )
    }

    pub fn get_max(&self) -> Vector2<f32> {
        Vector2::new(
            self.rigidbody2d.get_position().x + self.half_size.x,
            self.rigidbody2d.get_position().y + self.half_size.y,
        )
    }

    pub fn get_vertices(&self) -> Vec<Vector2<f32>> {
        let min = self.get_min();
        let max = self.get_max();

        let mut vertices = vec![
            Vector2::new(min.x, min.y),
            Vector2::new(min.x, max.y),
            Vector2::new(max.x, min.y),
            Vector2::new(max.x, max.y),
        ];

        if self.rigidbody2d.get_rotation() != 0.0 {}

        vertices
    }
}

impl Component for BoxCollider2D {
    type Storage = DenseVecStorage<Self>;
}

pub struct AABB {
    center: Vector2<f32>,
    size: Vector2<f32>,
    half_size: Vector2<f32>,
    rigidbody2d: Rigidbody2D,
}

impl AABB {
    pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        let size = Vector2::new(max.x - min.x, max.y - min.y);
        let center = Vector2::new(min.x + size.x, min.y + size.y).mul(0.5);
        AABB {
            center: center,
            size: size,
            half_size: size.mul(0.5),
            rigidbody2d: Rigidbody2D::new(),
        }
    }

    pub fn get_min(&self) -> Vector2<f32> {
        Vector2::new(
            self.rigidbody2d.get_position().x - self.half_size.x,
            self.rigidbody2d.get_position().y - self.half_size.y,
        )
    }

    pub fn get_max(&self) -> Vector2<f32> {
        Vector2::new(
            self.rigidbody2d.get_position().x + self.half_size.x,
            self.rigidbody2d.get_position().y + self.half_size.y,
        )
    }
}
