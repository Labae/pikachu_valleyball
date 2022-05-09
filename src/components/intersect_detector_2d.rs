use amethyst::core::math::{Matrix3, Vector2};

use super::primitives::{BoxCollider2D, Circle, AABB};

pub struct IntersectDetector2D;

impl IntersectDetector2D {
    pub fn point_in_circle(point: Vector2<f32>, circle: Circle) -> bool {
        let circle_center = circle.get_center();
        let circle_to_point = Vector2::new(point.x - circle_center.x, point.y - circle_center.y);

        circle_to_point.magnitude_squared() <= circle.get_radius() * circle.get_radius()
    }

    pub fn point_in_aabb(point: Vector2<f32>, aabb: AABB) -> bool {
        let min = aabb.get_min();
        let max = aabb.get_max();

        point.x <= max.x && min.x <= point.x && point.y <= max.y && min.y <= point.y
    }

    pub fn point_in_box2d(point: Vector2<f32>, box2d: BoxCollider2D) -> bool {
        let max = box2d.get_max();
        let min = box2d.get_min();

        point.x <= max.x && min.x <= point.x && point.y <= max.y && min.y <= point.y
    }
}
