use bevy::prelude::{App, Plugin};

pub struct CreepPlugin;

impl CreepPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CreepPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {}
}
