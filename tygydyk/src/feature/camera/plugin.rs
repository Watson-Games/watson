use crate::feature::camera::system::setup_camera;
use crate::feature::camera::TygydykCamera;
use bevy::prelude::{App, Plugin, Startup};

#[derive(Default)]
pub struct TygydykCameraPlugin;

impl Plugin for TygydykCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TygydykCamera>()
            .add_systems(Startup, setup_camera);
    }
}
