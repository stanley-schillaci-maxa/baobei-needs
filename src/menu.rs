/// Systems of the menu phase.
use bevy::{
    input::{keyboard::KeyboardInput, system::exit_on_esc_system, ElementState},
    prelude::*,
};

use crate::constants::{GameState, STAGE};

/// Plugin managing contact collisions
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MenuMaterials>()
            .on_state_enter(STAGE, GameState::Menu, setup_menu.system())
            .on_state_update(STAGE, GameState::Menu, button_system.system())
            .on_state_update(STAGE, GameState::Menu, play_on_space_system.system())
            .on_state_update(STAGE, GameState::Menu, exit_on_esc_system.system())
            .on_state_exit(STAGE, GameState::Menu, cleanup_menu.system());
    }
}

/// Stores entities in the menu phase
struct MenuData {
    /// Entity wrapping all menu entities (title, buttons)
    node_wrapper: Entity,
}

/// Colors of the button.
struct MenuMaterials {
    /// Transparent color
    none: Handle<ColorMaterial>,
    /// Default style of a button
    normal_button: Handle<ColorMaterial>,
    /// Hovered style of a button
    hovered_button: Handle<ColorMaterial>,
}

impl FromResources for MenuMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            none: materials.add(Color::NONE.into()),
            normal_button: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered_button: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

/// A button interacted by the player.
type UpdatedButton = (Mutated<Interaction>, With<Button>);

/// Handles clicks on the `Play` button.
fn button_system(
    materials: Res<MenuMaterials>,
    mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>), UpdatedButton>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => state.set_next(GameState::InGame).unwrap(),
            Interaction::Hovered => *material = materials.hovered_button.clone(),
            Interaction::None => *material = materials.normal_button.clone(),
        }
    }
}

/// Setup the title and `Play` button in the main menu.
fn setup_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
    commands.spawn(CameraUiBundle::default());

    let font = asset_server.load("FiraSans-Bold.ttf");

    let node_wrapper = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: Rect::all(Val::Px(50.0)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Style::default()
            },
            material: materials.none.clone(),
            ..NodeBundle::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "Baobei needs".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 125.0,
                            color: Color::WHITE,
                            ..TextStyle::default()
                        },
                    },
                    ..TextBundle::default()
                })
                .spawn(ButtonBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(25.0)),
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center, // horizontally center child text
                        align_items: AlignItems::Center,         // vertically center child text
                        ..Style::default()
                    },
                    material: materials.normal_button.clone(),
                    ..ButtonBundle::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            value: "Play".to_string(),
                            font: font.clone(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..TextStyle::default()
                            },
                        },
                        ..TextBundle::default()
                    });
                });
        })
        .current_entity()
        .unwrap();

    commands.insert_resource(MenuData { node_wrapper });
}

/// Removes all entities of the menu.
fn cleanup_menu(commands: &mut Commands, menu_data: Res<MenuData>) {
    commands.despawn_recursive(menu_data.node_wrapper);
}

/// Start the game play when the player press `Space`.
fn play_on_space_system(
    mut keyboard_input_reader: Local<EventReader<KeyboardInput>>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    mut state: ResMut<State<GameState>>,
) {
    for event in keyboard_input_reader.iter(&keyboard_input_events) {
        let space_pressed = matches!(event, KeyboardInput {
            key_code: Some(KeyCode::Space),
            state: ElementState::Pressed,
            ..
        });

        if space_pressed {
            state.set_next(GameState::InGame).unwrap();
        }
    }
}
