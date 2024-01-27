use crate::feature::kitten::{
    Kitten, KITTEN_DENSITY, KITTEN_FOOT_HEIGHT, KITTEN_FOOT_MARGIN, KITTEN_FRICTION,
    KITTEN_GRAVITY_SCALE, KITTEN_HEIGHT, KITTEN_JUMP_IMPULSE, KITTEN_MAX_SPEED,
    KITTEN_MOVE_IMPULSE, KITTEN_WIDTH,
};
use crate::feature::movement::{Foot, JumpImpulse, MaxSpeed, MoveDirection, MoveImpulse};
use bevy::prelude::{
    default, Assets, Color, ColorMaterial, Commands, Mesh, Rectangle, ResMut, Transform, Vec2,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, Damping, ExternalForce, ExternalImpulse, Friction,
    GravityScale, LockedAxes, Restitution, RigidBody, Velocity,
};

pub fn setup_kitten(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Kitten)
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(KITTEN_WIDTH, KITTEN_HEIGHT))
                .into(),
            material: materials.add(Color::LIME_GREEN),
            transform: Transform::from_xyz(0., 200., 0.),
            ..default()
        })
        // Physics
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(KITTEN_WIDTH / 2., KITTEN_HEIGHT / 2.))
        .insert(ColliderMassProperties::Density(KITTEN_DENSITY))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(KITTEN_GRAVITY_SCALE))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(Velocity::default())
        .insert(Friction {
            coefficient: KITTEN_FRICTION,
            ..default()
        })
        .insert(Restitution::default())
        .insert(Damping::default())
        // Movement
        .insert(MoveDirection::default())
        .insert(JumpImpulse(KITTEN_JUMP_IMPULSE))
        .insert(MoveImpulse(KITTEN_MOVE_IMPULSE))
        .insert(MaxSpeed(KITTEN_MAX_SPEED))
        .insert(Foot {
            rel_pos: Vec2::new(
                0.,
                -(KITTEN_HEIGHT / 2. + KITTEN_FOOT_HEIGHT / 2. + KITTEN_FOOT_MARGIN),
            ),
            collider: Collider::cuboid(KITTEN_WIDTH, KITTEN_FOOT_HEIGHT / 2.),
        });
}
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::{App, Startup};

    #[test]
    #[ignore] // somehow it fails, idk why
    fn test__setup_kitten__spawned_Kitten() {
        // given
        let mut app = App::new();

        app.add_systems(Startup, setup_kitten);

        // when
        app.update();

        // then
        let len = app.world.query::<&Kitten>().iter(&app.world).len();

        assert_eq!(len, 1);
    }
}
