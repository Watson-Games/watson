use crate::util::{GAME_NAME, ORGANISATION_NAME};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{App, Plugin};
use bevy::MinimalPlugins;
use bevy_pkv::PkvStore;
use bevy_rapier3d::prelude::RapierDebugRenderPlugin;

pub struct TygydykServerPlugin;

impl TygydykServerPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TygydykServerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for TygydykServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins)
            .insert_resource(PkvStore::new(ORGANISATION_NAME, GAME_NAME));

        if cfg!(debug_assertions) {
            app.add_plugins(FrameTimeDiagnosticsPlugin)
                .add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}
