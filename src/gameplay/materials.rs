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
}

impl FromResources for GameplayMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();

        Self {
            none: materials.add(Color::NONE.into()),
            didi_sprite: materials.add(asset_server.load("didi.png").into()),
            background_sprite: materials.add(asset_server.load("background.png").into()),
            baobei_sprite: materials.add(asset_server.load("baobei.png").into()),
            ice_cream_sprite: materials.add(asset_server.load("items/ice_cream.png").into()),
            water_glass_sprite: materials.add(asset_server.load("items/water_glass.png").into()),
            chips_sprite: materials.add(asset_server.load("items/chips.png").into()),
            fridge_sprite: materials.add(asset_server.load("furniture/fridge.png").into()),
            couch_sprite: materials.add(asset_server.load("furniture/couch.png").into()),
            kitchen_sprite: materials.add(asset_server.load("furniture/kitchen.png").into()),
            sink_sprite: materials.add(asset_server.load("furniture/sink.png").into()),
            table_sprite: materials.add(asset_server.load("furniture/table.png").into()),
        }
    }
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
