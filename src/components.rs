use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(0., 0.),
            mass: 1.0,
            radius: 0.0,
        }
    }
}
