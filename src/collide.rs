//! Trait for enabling collision on types

use super::prelude::*;

macro_rules! reflexive_collision {
    [$T:ty, $U:ty] => {
        impl CollideWith<$T> for $U {
            type Output = <$T as CollideWith<$U>>::Output;

            fn check_collision(&self, other: &$T) -> Self::Output {
                other.check_collision(self)
            }
        }
    };
}

/// Defines how an object collides with other objects
pub trait CollideWith<T> {
    /// Information about the collision
    type Output;

    /// Checks if the two objects are overlapping
    fn check_collision(&self, other: &T) -> Self::Output;
}

impl CollideWith<Rectangle> for Rectangle {
    type Output = bool;

    fn check_collision(&self, other: &Rectangle) -> Self::Output {
        self.check_collision_recs(other)
    }
}

reflexive_collision![Rectangle, Vector2];
impl CollideWith<Vector2> for Rectangle {
    type Output = bool;

    fn check_collision(&self, other: &Vector2) -> Self::Output {
        self.check_collision_point_rec(other)
    }
}

/// A circle
pub struct Circle {
    center: Vector2,
    radius: f32,
}

impl CollideWith<Circle> for Circle {
    type Output = bool;

    fn check_collision(&self, other: &Circle) -> Self::Output {
        check_collision_circles(self.center, self.radius, other.center, other.radius)
    }
}

reflexive_collision![Circle, Vector2];
impl CollideWith<Vector2> for Circle {
    type Output = bool;

    fn check_collision(&self, other: &Vector2) -> Self::Output {
        check_collision_point_circle(other, self.center, self.radius)
    }
}

/// A triangle
pub struct Triangle {
    p: [Vector2; 3],
}

impl CollideWith<Vector2> for Triangle {
    type Output = bool;

    fn check_collision(&self, other: &Vector2) -> Self::Output {
        check_collision_point_triangle(other, self.p[0], self.p[1], self.p[2])
    }
}
