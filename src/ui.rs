use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct CursorCoords(pub Vec2);

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FpsText;

pub fn cursor_coords_system(
    mut my_coords: ResMut<CursorCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_coords_text: Query<&mut Text, With<FpsText>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    let mut coords_text = q_coords_text.single_mut();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if my_coords.0 == world_position {
            return;
        };
        my_coords.0 = world_position;

        coords_text.sections[1].value = format!("{:.1}/{:.1}", world_position.x, world_position.y);
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings {
            intensity: 0.4,
            ..default()
        },
        MainCamera,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Coords: ",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        FpsText,
    ));
}
