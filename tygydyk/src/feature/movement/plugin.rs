use crate::feature::movement::system::{
    handle_brake, handle_jump, handle_move, update_grounded, update_move_direction,
};
use crate::feature::movement::{Grounded, JumpImpulse, MaxSpeed, MoveDirection, MoveImpulse};
use bevy::prelude::{App, Plugin, Update};

pub struct MovementPlugin;

impl MovementPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MovementPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Grounded>()
            .register_type::<JumpImpulse>()
            .register_type::<MaxSpeed>()
            .register_type::<MoveDirection>()
            .register_type::<MoveImpulse>()
            .add_systems(
                Update,
                (
                    handle_jump,
                    handle_move,
                    handle_brake,
                    update_move_direction,
                    update_grounded,
                ),
            );
    }
}
