use bevy::ecs::system::SystemParam;
use bevy_rapier3d::prelude::BevyPhysicsHooks;

#[derive(SystemParam)]
pub struct TygydykPhysicsHooks;

impl BevyPhysicsHooks for TygydykPhysicsHooks {}
