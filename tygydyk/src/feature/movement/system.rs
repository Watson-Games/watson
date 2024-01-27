use crate::feature::input::InputAction;
use crate::feature::kitten::{Kitten, KITTEN_FRICTION};
use crate::feature::movement::{Foot, Grounded, JumpImpulse, MaxSpeed, MoveDirection, MoveImpulse};
use bevy::prelude::{Commands, Entity, Query, Res, Transform, Vec3Swizzles, With};
use bevy_rapier2d::prelude::{ExternalImpulse, Friction, QueryFilter, RapierContext, Velocity};
use leafwing_input_manager::prelude::ActionState;

pub fn handle_brake(
    mut q: Query<&mut Friction, With<Kitten>>,
    action_state: Res<ActionState<InputAction>>,
) {
    for mut friction in &mut q {
        if action_state.just_pressed(&InputAction::Brake) {
            friction.coefficient = KITTEN_FRICTION * 3.;
        }

        if action_state.just_released(&InputAction::Brake) {
            friction.coefficient = KITTEN_FRICTION;
        }
    }
}

pub fn handle_jump(
    mut q: Query<(&mut ExternalImpulse, &JumpImpulse, Option<&Grounded>), With<Kitten>>,
    action_state: Res<ActionState<InputAction>>,
) {
    if !action_state.pressed(&InputAction::Jump) {
        return;
    }

    for (mut ei, ji, grounded) in &mut q {
        // Unable to jump
        if grounded.is_none() {
            continue;
        }

        ei.impulse.y = ji.0;
    }
}

pub fn handle_move(
    mut q: Query<
        (
            &mut ExternalImpulse,
            &Velocity,
            &MaxSpeed,
            &MoveImpulse,
            Option<&Grounded>,
        ),
        With<Kitten>,
    >,
    action_state: Res<ActionState<InputAction>>,
) {
    if !action_state.pressed(&InputAction::Move) {
        return;
    }

    let axis_pair = action_state
        .clamped_axis_pair(&InputAction::Move)
        .expect("Unable to get movement axis");

    if axis_pair.length_squared() < 0. {
        return;
    }

    if axis_pair.y() < 0. {
        // brake pressed
        return;
    }

    for (mut ei, vel, ms, mi, grounded) in &mut q {
        let mi = if grounded.is_some() { mi.0 } else { mi.0 / 2. };

        if vel.linvel.x < -ms.0 && axis_pair.x() < 0. || vel.linvel.x > ms.0 && axis_pair.x() > 0. {
            continue;
        }

        ei.impulse.x += axis_pair.x() * mi;
    }
}

pub fn update_grounded(
    mut commands: Commands,
    q: Query<(Entity, &Transform, &Foot, Option<&Grounded>), With<Kitten>>,
    rapier_context: Res<RapierContext>,
) {
    for (entity_id, transform, foot, grounded) in &q {
        let delta_grounded = rapier_context
            .intersection_with_shape(
                transform.translation.xy() + foot.rel_pos,
                0.,
                &foot.collider,
                QueryFilter::default().exclude_sensors(),
            )
            .is_some();

        let grounded = grounded.is_some();

        if grounded && !delta_grounded {
            commands.entity(entity_id).remove::<Grounded>();
        } else if !grounded && delta_grounded {
            commands.entity(entity_id).insert(Grounded);
        }
    }
}

pub fn update_move_direction(
    mut q: Query<&mut MoveDirection, With<Kitten>>,
    action_state: Res<ActionState<InputAction>>,
) {
    if !action_state.pressed(&InputAction::Move) {
        return;
    }

    let axis_pair = action_state
        .clamped_axis_pair(&InputAction::Move)
        .expect("Unable to get movement axis");

    if axis_pair.length_squared() < 0. {
        return;
    }

    for mut move_dir in &mut q {
        *move_dir = if axis_pair.x() > 0. {
            MoveDirection::Right
        } else if axis_pair.x() < 0. {
            MoveDirection::Left
        } else {
            *move_dir
        };
    }
}
