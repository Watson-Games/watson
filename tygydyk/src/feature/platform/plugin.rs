use crate::feature::platform::system::setup_platform;
use crate::feature::platform::Platform;
use bevy::prelude::{App, Plugin, Startup};

#[derive(Default)]
pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Platform>()
            .add_systems(Startup, setup_platform);
    }
}
