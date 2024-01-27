use crate::feature::input::InputAction;
use bevy::prelude::{App, Plugin};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputAction>::default())
            .init_resource::<ActionState<InputAction>>()
            .insert_resource(InputAction::default_input_map());
    }
}
