use crate::feature::creep::CreepPlugin;
use crate::feature::ding_ding::DingDingPlugin;
use crate::feature::map::MapPlugin;
use crate::feature::stranger::StrangerPlugin;
use bevy::prelude::{App, Plugin};
use bevy::DefaultPlugins;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

pub struct SurroundedPlugin;

impl SurroundedPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SurroundedPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for SurroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(CreepPlugin)
            .add_plugins(DingDingPlugin)
            .add_plugins(MapPlugin)
            .add_plugins(StrangerPlugin);

        if cfg!(debug_assertions) {
            app.add_plugins(RapierDebugRenderPlugin::default())
                .add_plugins(EditorPlugin::default());
        }
    }
}
