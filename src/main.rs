use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

mod camera;
mod input;
mod items;
mod movement;
mod player;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    PauseMenu,
    Gameplay,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 3rd Party Plugins
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            InputManagerPlugin::<player::input::PlayerAction>::default(),
        ))
        // Project Plugins
        .add_plugins((player::PlayerPlugin, camera::CameraPlugin))
        .insert_resource(RapierConfiguration {
            gravity: Vec3::Y * -30.0,
            ..default()
        })
        .run();
}
