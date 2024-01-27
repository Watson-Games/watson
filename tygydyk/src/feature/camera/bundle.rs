use crate::feature::camera::{TygydykCamera, CAMERA_FAR, CAMERA_NEAR};
use bevy::prelude::{default, Bundle, Camera2dBundle, OrthographicProjection};

#[derive(Bundle)]
pub struct TygydykCameraBundle {
    _m: TygydykCamera,
    camera: Camera2dBundle,
}

impl TygydykCameraBundle {
    pub fn new() -> Self {
        Self {
            _m: TygydykCamera,
            camera: Camera2dBundle {
                projection: OrthographicProjection {
                    far: CAMERA_FAR,
                    near: CAMERA_NEAR,
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl Default for TygydykCameraBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::camera::TygydykCameraBundle;

    #[test]
    fn test__TygydykCameraBundle__new() {
        // given
        let expected_far = 1.0f32;
        let expected_near = -1.0f32;

        // when
        let bundle = TygydykCameraBundle::new();

        // then
        assert!((expected_far - bundle.camera.projection.far).abs() < f32::EPSILON);
        assert!((expected_near - bundle.camera.projection.near).abs() < f32::EPSILON);
    }
}
