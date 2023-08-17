use crate::{
    player::{input::PlayerAction, Player},
    GameState,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_camera)
            .add_systems(
                Update,
                (
                    target_player,
                    read_rotation_inputs,
                    position_and_rotate_camera,
                )
                    .chain()
                    .run_if(in_state(GameState::Gameplay)),
            );
    }
}

#[derive(Component)]
#[allow(dead_code)]
pub struct PrimaryCamera {
    x_angle: f32,
    y_angle: f32,
    fov_degrees: f32,
    target: Vec3,
    offset: Vec3,
    translation_easing: f32,
    rotation_easing: f32,
}

#[allow(dead_code)]
impl PrimaryCamera {
    pub fn adjust_x_angle(&mut self, change: f32) {
        self.x_angle = (self.x_angle + change).clamp(-87.0, 87.0);
    }

    pub fn adjust_y_angle(&mut self, change: f32) {
        self.y_angle += change;
    }

    pub fn adjust_fov(&mut self, change: f32) {
        self.fov_degrees += change;
    }

    pub fn adjust_target(&mut self, change: Vec3) {
        self.target += change;
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    pub fn adjust_offset(&mut self, change: Vec3) {
        self.offset += change;
    }

    pub fn set_offset(&mut self, offset: Vec3) {
        self.offset = offset;
    }
}

impl Default for PrimaryCamera {
    fn default() -> Self {
        PrimaryCamera {
            x_angle: 0.0,
            y_angle: 0.0,
            fov_degrees: 45.0,
            target: Vec3::ZERO,
            offset: Vec3::Y,
            translation_easing: 20.0,
            rotation_easing: 20.0,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), PrimaryCamera::default()));
}

fn target_player(
    mut camera_query: Query<&mut PrimaryCamera, Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            camera.target = player_transform.translation;
        }
    }
}

fn read_rotation_inputs(
    mut camera_query: Query<&mut PrimaryCamera>,
    player_inputs_query: Query<&ActionState<PlayerAction>>,
    time: Res<Time>,
) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        if let Ok(action) = player_inputs_query.get_single() {
            if action.pressed(PlayerAction::Pan) {
                let camera_pan_vector = action.axis_pair(PlayerAction::Pan).unwrap();

                let y_change = if camera_pan_vector.x() != 0.0 {
                    15.0 * camera_pan_vector.x() * time.delta_seconds()
                } else {
                    0.0
                };

                let x_change = if camera_pan_vector.y() != 0.0 {
                    15.0 * camera_pan_vector.y() * time.delta_seconds()
                } else {
                    0.0
                };

                if x_change != 0.0 {
                    camera.adjust_x_angle(-x_change);
                }

                if y_change != 0.0 {
                    camera.adjust_y_angle(-y_change);
                }
            }
        }
    }
}

fn position_and_rotate_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &PrimaryCamera)>,
) {
    if let Ok((mut transform, camera)) = camera_query.get_single_mut() {
        let mut starting_transform = Transform::from_translation(camera.target);

        let x_angle = camera.x_angle.to_radians();
        let y_angle = camera.y_angle.to_radians();

        starting_transform.rotate_y(y_angle);

        let desired_position = starting_transform.translation + (Vec3::Y * camera.offset.y);

        let mut desired_rotation = Transform::default();

        desired_rotation.rotate_x(x_angle);
        desired_rotation.rotate_y(y_angle);

        let slerp_rotation = transform.rotation.slerp(
            desired_rotation.rotation,
            time.delta_seconds() * camera.rotation_easing,
        );

        let lerp_position = transform.translation.lerp(
            desired_position,
            time.delta_seconds() * camera.translation_easing,
        );

        transform.translation = lerp_position;
        transform.rotation = slerp_rotation;
    }
}
