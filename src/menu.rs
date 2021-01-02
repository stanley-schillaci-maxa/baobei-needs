/// Systems of the menu phase.
use bevy::{input::system::exit_on_esc_system, prelude::*};

use crate::constants::{GameState, STAGE};

/// Plugin managing contact collisions
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .on_state_enter(STAGE, GameState::Menu, setup_menu.system())
            .on_state_update(STAGE, GameState::Menu, button_system.system())
            .on_state_update(STAGE, GameState::Menu, exit_on_esc_system.system())
            .on_state_exit(STAGE, GameState::Menu, cleanup_menu.system());
    }
}

/// Entities in the menu phase
struct MenuData {
    /// Entity of the main title
    title: Entity,
    /// Entity of the `Play` button
    button: Entity,
}

/// Colors of the button.
struct ButtonMaterials {
    /// Default style
    normal: Handle<ColorMaterial>,
    /// Hover style
    hovered: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

/// A button interacted by the player.
type UpdatedButton = (Mutated<Interaction>, With<Button>);

/// Handles clicks on the `Play` button.
fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>), UpdatedButton>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => state.set_next(GameState::InGame).unwrap(),
            Interaction::Hovered => *material = button_materials.hovered.clone(),
            Interaction::None => *material = button_materials.normal.clone(),
        }
    }
}

/// Setup the camera define the text for the main menu.
fn setup_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    let font = asset_server.load("FiraSans-Bold.ttf");

    commands
        // UI camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,

                ..Style::default()
            },
            text: Text {
                value: "Baobei needs".to_string(),
                font: font.clone(),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..TextStyle::default()
                },
            },
            ..TextBundle::default()
        });
    let title_entity = commands.current_entity().unwrap();

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Style::default()
            },
            material: button_materials.normal.clone(),
            ..ButtonBundle::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Play".to_string(),
                    font: font.clone(),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..TextStyle::default()
                    },
                },
                ..TextBundle::default()
            });
        });
    let button_entity = commands.current_entity().unwrap();

    commands.insert_resource(MenuData {
        title: title_entity,
        button: button_entity,
    });
}

/// Removes all entities of the menu.
fn cleanup_menu(commands: &mut Commands, menu_data: Res<MenuData>) {
    commands.despawn_recursive(menu_data.title);
    commands.despawn_recursive(menu_data.button);
}
