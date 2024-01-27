use crate::feature::platform::Platform;
use crate::util::PIXELS_PER_METER;
use bevy::prelude::{
    default, Assets, Color, ColorMaterial, Commands, Mesh, Rectangle, ResMut, Transform,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub fn setup_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Platform)
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(
                    1000. * PIXELS_PER_METER,
                    1. * PIXELS_PER_METER,
                ))
                .into(),
            material: materials.add(Color::GRAY),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        // Physics
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            500. * PIXELS_PER_METER,
            0.5 * PIXELS_PER_METER,
        ));
}
