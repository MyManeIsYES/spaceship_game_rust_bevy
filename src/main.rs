mod camera;
mod debug;
mod movement;
mod spaceship;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;
mod schedule;
mod state;
mod health;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 900.0,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(DespawnPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}




