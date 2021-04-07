//! Loads sprites and materials used in the game.

use bevy::prelude::*;

use super::items::Item;

/// Sprites and colors in the gameplay phase.
pub struct GameplayMaterials {
    /// Transparent color
    pub none: Handle<ColorMaterial>,
    /// Sprite of the background image
    pub background_sprite: Handle<ColorMaterial>,
    /// Sprite of didi
    pub didi_sprite: Handle<ColorMaterial>,
    /// Sprite of baobei
    pub baobei_sprite: Handle<ColorMaterial>,
    /// Sprite for the ice cream item
    pub ice_cream_sprite: Handle<ColorMaterial>,
    /// Sprite for the water glass item
    pub water_glass_sprite: Handle<ColorMaterial>,
    /// Sprite for the chips item
    pub chips_sprite: Handle<ColorMaterial>,
    /// Sprite for the fridge
    pub fridge_sprite: Handle<ColorMaterial>,
    /// Sprite for the couch
    pub couch_sprite: Handle<ColorMaterial>,
    /// Sprite for the kitchen
    pub kitchen_sprite: Handle<ColorMaterial>,
    /// Sprite for the sink
    pub sink_sprite: Handle<ColorMaterial>,
    /// Sprite for the table
    pub table_sprite: Handle<ColorMaterial>,
    /// Texture atlas for emotions sprites
    pub emotion_atlas: Handle<TextureAtlas>,
}

impl FromWorld for GameplayMaterials {
    fn from_world(world: &mut World) -> Self {
        let none = {
            let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
            materials.add(Color::NONE.into())
        };

        let emotion_atlas = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();
            let sprite = asset_server.load("emotions.png");
            let atlas = TextureAtlas::from_grid(sprite, Vec2::new(152.0, 152.0), 5, 1);

            let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
            texture_atlases.add(atlas)
        };

        Self {
            none,
            didi_sprite: load_sprite(world, "didi.png"),
            background_sprite: load_sprite(world, "background.png"),
            baobei_sprite: load_sprite(world, "baobei.png"),
            ice_cream_sprite: load_sprite(world, "items/ice_cream.png"),
            water_glass_sprite: load_sprite(world, "items/water_glass.png"),
            chips_sprite: load_sprite(world, "items/chips.png"),
            fridge_sprite: load_sprite(world, "furniture/fridge.png"),
            couch_sprite: load_sprite(world, "furniture/couch.png"),
            kitchen_sprite: load_sprite(world, "furniture/kitchen.png"),
            sink_sprite: load_sprite(world, "furniture/sink.png"),
            table_sprite: load_sprite(world, "furniture/table.png"),
            emotion_atlas,
        }
    }
}

/// Load the sprite in the given file.
fn load_sprite(world: &mut World, file_name: &str) -> Handle<ColorMaterial> {
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let sprite = asset_server.load(file_name).into();

    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
    materials.add(sprite)
}

impl GameplayMaterials {
    /// Returns the sprite handle for the given item
    pub fn item_sprite_for(&self, item: Item) -> Handle<ColorMaterial> {
        match item {
            Item::IceCream => self.ice_cream_sprite.clone(),
            Item::WaterGlass => self.water_glass_sprite.clone(),
            Item::Chips => self.chips_sprite.clone(),
        }
    }
}
