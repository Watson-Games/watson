use bevy::app::App;
use bevy::prelude::Plugin;
use bevy::DefaultPlugins;
use bevy_editor_pls::EditorPlugin;

pub struct UntitledPlugin;

impl UntitledPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UntitledPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for UntitledPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);

        if cfg!(debug_assertions) {
            app.add_plugins(EditorPlugin::default());
        }
    }
}
