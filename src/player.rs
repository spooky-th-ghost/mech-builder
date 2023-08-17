use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        crate::movement::MovementBundle::default()
            .with_translation(Vec3::Y)
            .with_collider(Collider::capsule_y(1.0, 0.5))
            .with_damping(Damping {
                linear_damping: 0.2,
                angular_damping: 0.0,
            })
            .with_friction(Friction {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            }),
        Player,
        input::InputListenerBundle::input_map(),
    ));
}

pub mod input {
    use bevy::prelude::*;
    use leafwing_input_manager::{prelude::*, *};

    #[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default, Reflect)]
    pub enum PlayerAction {
        #[default]
        Pan,
        PanGamepad,
        Jump,
        Move,
        Use,
    }

    #[derive(Bundle)]
    pub struct InputListenerBundle {
        input_manager: InputManagerBundle<PlayerAction>,
    }

    impl InputListenerBundle {
        pub fn input_map() -> InputListenerBundle {
            use PlayerAction::*;

            let input_map = input_map::InputMap::new([(KeyCode::Space, Jump)])
                .insert(MouseButton::Left, Use)
                .insert_multiple([
                    (DualAxis::mouse_motion(), Pan),
                    (DualAxis::right_stick(), PanGamepad),
                    (DualAxis::left_stick(), Move),
                ])
                .insert(VirtualDPad::wasd(), Move)
                .set_gamepad(Gamepad { id: 1 })
                .build();

            InputListenerBundle {
                input_manager: InputManagerBundle {
                    input_map,
                    ..Default::default()
                },
            }
        }
    }
}
