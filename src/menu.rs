//! Systems of the menu phase.

use bevy::{input::system::exit_on_esc_system, prelude::*};

use crate::constants::GameState;

/// Plugin managing contact collisions
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MenuMaterials>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(button_system.system())
                    .with_system(play_on_space_system.system())
                    .with_system(exit_on_esc_system.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu.system()));
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

impl FromWorld for MenuMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            none: materials.add(Color::NONE.into()),
            normal_button: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered_button: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

/// A button interacted by the player.
type UpdatedButton = (Changed<Interaction>, With<Button>);

/// Handles clicks on the `Play` button.
fn button_system(
    materials: Res<MenuMaterials>,
    mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>), UpdatedButton>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => state.set(GameState::InGame).unwrap(),
            Interaction::Hovered => *material = materials.hovered_button.clone(),
            Interaction::None => *material = materials.normal_button.clone(),
        }
    }
}

/// Setup the title and `Play` button in the main menu.
fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
    let font = asset_server.load("FiraSans-Bold.ttf");

    let node_wrapper = commands
        .spawn_bundle(NodeBundle {
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
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Baobei needs",
                    TextStyle {
                        font: font.clone(),
                        font_size: 125.0,
                        color: Color::WHITE,
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            });
            parent
                .spawn_bundle(ButtonBundle {
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
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play",
                            TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                            TextAlignment::default(),
                        ),
                        ..TextBundle::default()
                    });
                });
        })
        .id();

    commands.insert_resource(MenuData { node_wrapper });
}

/// Removes all entities of the menu.
fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.node_wrapper).despawn_recursive();
}

/// Start the game play when the player press `Space`.
fn play_on_space_system(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::InGame).unwrap();
    }
}
