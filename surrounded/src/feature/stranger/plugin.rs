use bevy::asset::Assets;
use bevy::hierarchy::BuildChildren;
use bevy::math::EulerRot;
use bevy::pbr::StandardMaterial;
use bevy::prelude::Projection::Perspective;
use bevy::prelude::{
    default, shape, App, Camera3dBundle, Color, Commands, Component, Mesh, PbrBundle,
    PerspectiveProjection, Plugin, Projection, Quat, Query, Reflect, Res, ResMut, SpatialBundle,
    Startup, Time, Transform, Update, Vec3, Visibility, With,
};
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};
use leafwing_input_manager::orientation::Orientation;
use leafwing_input_manager::prelude::{
    ActionState, DualAxis, InputManagerPlugin, InputMap, QwertyScanCode,
};
use leafwing_input_manager::{Actionlike, InputManagerBundle};

pub struct StrangerPlugin;

impl StrangerPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StrangerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for StrangerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<StrangerAction>::default())
            .add_systems(Startup, setup_stranger)
            .add_systems(Update, (move_stranger, rotate_camera));
    }
}

#[derive(Component)]
struct Stranger;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum StrangerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    LookAround,
}

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct MenuCamera;

#[derive(Component)]
struct Sensitivity(f32);

impl Sensitivity {
    pub fn new() -> Self {
        Self(30.)
    }
}

impl Default for Sensitivity {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_stranger(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Stranger)
        .insert(SpatialBundle {
            transform: Transform::from_xyz(0., 1.4, 0.),
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::capsule_y(0.9, 0.5))
        .insert(KinematicCharacterController::default())
        .insert(InputManagerBundle::<StrangerAction> {
            input_map: InputMap::default()
                .insert(QwertyScanCode::W, StrangerAction::MoveForward)
                .insert(QwertyScanCode::S, StrangerAction::MoveBackward)
                .insert(QwertyScanCode::A, StrangerAction::MoveLeft)
                .insert(QwertyScanCode::D, StrangerAction::MoveRight)
                .insert(DualAxis::mouse_motion(), StrangerAction::LookAround)
                .build(),
            ..default()
        })
        .with_children(|child| {
            child
                .spawn(InGameCamera)
                .insert(Camera3dBundle {
                    projection: Perspective(PerspectiveProjection {
                        fov: f32::to_radians(90.),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0., 1.8, 0.),
                    ..default()
                })
                .insert(Sensitivity::default());
        })
        .with_children(|child| {
            child.spawn(PbrBundle {
                mesh: meshes.add(
                    shape::Capsule {
                        radius: 0.5,
                        depth: 1.8,
                        ..default()
                    }
                    .into(),
                ),
                material: materials.add(Color::GREEN.into()),
                ..default()
            });
        });
}

fn move_stranger(
    mut stranger_query: Query<
        (
            &mut KinematicCharacterController,
            &ActionState<StrangerAction>,
        ),
        With<Stranger>,
    >,
    camera_query: Query<&Transform, With<InGameCamera>>,
    time: Res<Time>,
) {
    for (mut stranger_character_controller, action_state) in stranger_query.iter_mut() {
        for camera_transform in camera_query.iter() {
            let move_direction = Vec3::new(
                movement_axis(
                    action_state,
                    StrangerAction::MoveRight,
                    StrangerAction::MoveLeft,
                ),
                0.,
                movement_axis(
                    action_state,
                    StrangerAction::MoveBackward,
                    StrangerAction::MoveForward,
                ),
            );

            let camera_rotation = camera_transform.rotation;

            let mut move_vector = (strafe_vector(&camera_rotation) * move_direction.x)
                + (forward_walk_vector(&camera_rotation) * move_direction.z);

            if move_vector.length_squared() > 0. {
                move_vector = move_vector.normalize();
            }

            stranger_character_controller.translation =
                Some(move_vector * 10. * time.delta_seconds());
        }
    }
}

fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::Z).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
    let f = forward_vector(rotation);
    let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
    f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
    // Rotate it 90 degrees to get the strafe direction
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vector(rotation))
        .normalize()
}

fn movement_axis(
    action_state: &ActionState<StrangerAction>,
    plus: StrangerAction,
    minus: StrangerAction,
) -> f32 {
    if action_state.pressed(plus) {
        1.
    } else if action_state.pressed(minus) {
        -1.
    } else {
        0.
    }
}

fn rotate_camera(
    stranger_query: Query<&ActionState<StrangerAction>, With<Stranger>>,
    mut camera_query: Query<(&mut Transform, &Sensitivity, &Projection), With<InGameCamera>>,
    time: Res<Time>,
) {
    for action_state in stranger_query.iter() {
        for (mut camera_transform, sensitivity, projection) in camera_query.iter_mut() {
            let camera_rotate = action_state
                .axis_pair(StrangerAction::LookAround)
                .expect("Unable to get camera rotate vector!");

            if camera_rotate.length_squared() > 0. {
                let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

                pitch -= (sensitivity.0 * camera_rotate.y() * time.delta_seconds()).to_radians();

                match projection {
                    Perspective(perspective) => {
                        pitch = pitch.clamp(-perspective.fov, perspective.fov);
                    }
                    _ => {
                        panic!("Wrong camera projection!")
                    }
                }

                yaw -= (sensitivity.0 * camera_rotate.x() * time.delta_seconds()).to_radians();

                camera_transform
                    .rotation
                    .rotate_towards(Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.), None);
            }
        }
    }
}
