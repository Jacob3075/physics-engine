use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::components::Ball;
use crate::ui::{cursor_coords_system, setup_ui, CursorCoords};

mod components;
mod ui;

fn main() {
    App::new().add_plugins((DefaultPlugins, EngineSetup)).run();
}

pub struct EngineSetup;

impl Plugin for EngineSetup {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorCoords>();
        app.add_systems(Startup, setup_ui);
        app.add_systems(Startup, setup);
        app.add_systems(Update, cursor_coords_system);
        app.add_systems(Update, gravity_system);
        app.add_systems(Update, boundary_collision_system);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball {
            velocity: Default::default(),
            mass: 2.0,
            radius: 5.0,
        },
    ));
}

const GRAVITY: Vec2 = Vec2::new(0., -9.81);

fn gravity_system(mut query: Query<(&mut Transform, &mut Ball)>, time: Res<Time>) {
    for (mut transform, mut ball) in query.iter_mut() {
        let ball_mass = ball.mass;
        let force = ball_mass * GRAVITY;

        let velocity = ball.velocity + (force / ball_mass) * time.delta_seconds();
        let position = transform.translation.xy() + velocity * time.delta_seconds();
        ball.velocity = velocity;

        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

fn boundary_collision_system(
    mut query: Query<(&mut Transform, &mut Ball)>,
    window_q: Query<&Window>,
) {
    let window = window_q.single();
    for (mut transform, mut ball) in query.iter_mut() {
        let position = transform.translation.xy();
        let window_height = (window.height() / 2.) - ball.radius;
        let window_width = (window.width() / 2.) - ball.radius;

        // bottom of screen
        if position.y < -window_height {
            println!("at bottom");
            ball.velocity.y = -ball.velocity.y * 0.8;
            transform.translation.y = -window_height;
        }

        // top
        if position.y > window_height {
            println!("at top");
            ball.velocity.y = -ball.velocity.y * 0.8;
            ball.velocity.y = -ball.velocity.y * 0.8;
            transform.translation.y = window_height;
        }

        // left
        if position.x < -window_width {
            ball.velocity.x = -ball.velocity.x * 0.8;
            transform.translation.x = -window_width;
        }

        // right
        if position.x > window_width {
            ball.velocity.x = -ball.velocity.x * 0.8;
            transform.translation.x = window_width;
        }
    }
}
