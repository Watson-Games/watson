use crate::util::{GAME_NAME, ORGANISATION_NAME};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{App, Plugin};
use bevy::DefaultPlugins;
use bevy_editor_pls::EditorPlugin;
use bevy_pkv::PkvStore;

pub struct TygydykClientPlugin;

impl TygydykClientPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TygydykClientPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for TygydykClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .insert_resource(PkvStore::new(ORGANISATION_NAME, GAME_NAME));

        if cfg!(debug_assertions) {
            app.add_plugins(FrameTimeDiagnosticsPlugin)
                .add_plugins(EditorPlugin::default());
        }
    }
}
