use bevy::prelude::{Component, Reflect, Vec2};
use bevy_rapier2d::prelude::Collider;

#[derive(Component, Debug, Default)]
pub struct Foot {
    pub rel_pos: Vec2,
    pub collider: Collider,
}

#[derive(Component, Debug, Default, Reflect)]
pub struct Grounded;

#[derive(Component, Debug, Default, Reflect)]
pub struct JumpImpulse(pub f32);

#[derive(Component, Debug, Default, Reflect)]
pub struct MaxSpeed(pub f32);

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Reflect)]
pub enum MoveDirection {
    Left,
    Right,
}

impl Default for MoveDirection {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Component, Debug, Default, Reflect)]
pub struct MoveImpulse(pub f32);
