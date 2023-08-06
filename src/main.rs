use bevy::prelude::*;

mod camera;
mod input;
mod movement;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((player::PlayerPlugin, camera::CameraPlugin))
        .run();
}
