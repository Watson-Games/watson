use crate::feature::camera::TygydykCameraPlugin;
use crate::feature::input::InputPlugin;
use crate::feature::kitten::KittenPlugin;
use crate::feature::movement::MovementPlugin;
use crate::feature::platform::PlatformPlugin;
use crate::util::{AppState, TygydykPhysicsHooks, PIXELS_PER_METER};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{App, Plugin};
use bevy::DefaultPlugins;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::{RapierDebugRenderPlugin, RapierPhysicsPlugin};

pub struct TygydykPlugin;

impl TygydykPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TygydykPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for TygydykPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(
                RapierPhysicsPlugin::<TygydykPhysicsHooks>::pixels_per_meter(PIXELS_PER_METER),
            )
            // .insert_resource(PkvStore::new(ORGANISATION_NAME, GAME_NAME)) // Until bevy 0.13 isn't supported by bevy_pkv
            .init_state::<AppState>()
            .add_plugins(TygydykCameraPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(KittenPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(PlatformPlugin);

        if cfg!(debug_assertions) {
            app.add_plugins(FrameTimeDiagnosticsPlugin)
                .add_plugins(RapierDebugRenderPlugin::default())
                .add_plugins(EditorPlugin::default());
        }
    }
}
