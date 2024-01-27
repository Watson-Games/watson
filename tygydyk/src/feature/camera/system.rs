use crate::feature::camera::TygydykCameraBundle;
use bevy::prelude::Commands;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(TygydykCameraBundle::default());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::camera::TygydykCamera;
    use bevy::prelude::{App, Startup};

    #[test]
    fn test__setup_camera__spawned_TygydykCamera() {
        // given
        let mut app = App::new();

        app.add_systems(Startup, setup_camera);

        // when
        app.update();

        // then
        let len = app.world.query::<&TygydykCamera>().iter(&app.world).len();

        assert_eq!(len, 1);
    }
}
