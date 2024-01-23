use bevy::prelude::{App, Plugin};

pub struct DingDingPlugin;

impl DingDingPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DingDingPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for DingDingPlugin {
    fn build(&self, app: &mut App) {}
}
