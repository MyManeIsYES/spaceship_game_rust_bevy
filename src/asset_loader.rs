use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
           .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, assets_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: assets_server.load("Asteroid.glb#Scene0"),
        spaceship: assets_server.load("Spaceship.glb#Scene0"),
        missiles: assets_server.load("Missiles.glb#Scene0"),
    }
}
