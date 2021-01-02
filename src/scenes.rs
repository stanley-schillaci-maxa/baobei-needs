//! System that load the scene.

use std::path::PathBuf;

use bevy::prelude::*;

/// Plugin for managing the hot-loaded scene file.
pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<SpriteLoader>()
            .add_startup_system(load_scene_system.system())
            .add_system(load_sprite_system.system());
    }
}

/// Component indicating that a sprite will be loaded for the entity.
#[derive(Debug, Reflect, Default)]
#[reflect(Component)]
pub struct SpriteLoader {
    /// Path of the sprite to load
    pub path: String,
    /// Scale of the sprite
    pub scale: Vec3,
}

/// Hot reloads the scene file.
pub fn load_scene_system(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let scene_handle: Handle<DynamicScene> = asset_server.load("scene.scn");

    scene_spawner.spawn_dynamic(scene_handle);

    asset_server
        .watch_for_changes()
        .expect("Fail to hot load scene !");
}

/// Adds to entities with a `SpritLoader` the related `SpriteBundle`.
pub fn load_sprite_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &SpriteLoader), Added<SpriteLoader>>,
) {
    for (entity, sprite_loader) in query.iter() {
        commands.remove_one::<SpriteLoader>(entity);

        let path = PathBuf::from(sprite_loader.path.clone());

        commands.insert(
            entity,
            SpriteBundle {
                material: materials.add(asset_server.load(path).into()),
                transform: Transform::from_scale(sprite_loader.scale),
                ..SpriteBundle::default()
            },
        );
    }
}
