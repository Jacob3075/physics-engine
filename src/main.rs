mod components;
mod ui;

use crate::ui::{cursor_coords_system, setup_ui, CursorCoords};
use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, EngineSetup)).run();
}

pub struct EngineSetup;

impl Plugin for EngineSetup {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorCoords>();
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, cursor_coords_system);
    }
}
