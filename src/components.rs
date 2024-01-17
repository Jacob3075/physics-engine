use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Sphere {
    pub radius: f32,
}

impl Default for Sphere {
    fn default() -> Self {
        Self { radius: 0.0 }
    }
}

#[derive(Component)]
pub struct Body {
    pub velocity: Vec2,
    pub mass: f32,
    pub position: Vec2,
    pub restitution: f32,
}
impl Default for Body {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(0., 0.),
            mass: 1.0,
            position: Vec2::new(0., 0.),
            restitution: 1.0,
        }
    }
}
