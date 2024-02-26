use bevy::prelude::{KeyCode, Reflect};
use leafwing_input_manager::prelude::{InputKind, InputMap, UserInput, VirtualDPad};
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum InputAction {
    Brake,
    Move,
    Jump,
}

impl InputAction {
    pub fn default_input_map() -> InputMap<InputAction> {
        InputMap::<InputAction>::new([
            (Self::Move, UserInput::VirtualDPad(VirtualDPad::wasd())),
            (
                Self::Jump,
                UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
            ),
            (
                Self::Brake,
                UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyS)),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__InputAction__default_input_map() {
        // given
        let expected_input_map = InputMap::<InputAction>::new([
            (
                InputAction::Move,
                UserInput::VirtualDPad(VirtualDPad::wasd()),
            ),
            (
                InputAction::Jump,
                UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
            ),
            (
                InputAction::Brake,
                UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyS)),
            ),
        ]);

        // when
        let input_map = InputAction::default_input_map();

        // then
        assert_eq!(expected_input_map, input_map);
    }
}
