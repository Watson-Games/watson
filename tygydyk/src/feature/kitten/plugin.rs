use crate::feature::kitten::system::setup_kitten;
use crate::feature::kitten::Kitten;
use bevy::prelude::{App, Plugin, Startup};

#[derive(Default)]
pub struct KittenPlugin;

impl Plugin for KittenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Kitten>()
            .add_systems(Startup, setup_kitten);
    }
}
