use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
// use bevy_ecs_ldtk::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            // LDtk level selection resource
            // .insert_resource(LevelSelection::index(0))
            // .add_systems(OnEnter(GameState::Playing), setup_level)
            // .add_plugins(LdtkPlugin)
            .add_plugins(PanCamPlugin::default());
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

// fn setup_level(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
// ) {
//     // Change camera settings on playing state
//     info!("Change camera settings on playing state");
//     for (mut transform, mut projection) in &mut camera {
//         projection.scale = 0.5;
//         transform.translation.x += 1280.0 / 4.0;
//         transform.translation.y += 720.0 / 4.0;
//     }

//     // Spawn LDTK level
//     info!("Spawn LDTK level");
//     commands.spawn(LdtkWorldBundle {
//         ldtk_handle: asset_server.load("world.ldtk"),
//         ..Default::default()
//     });
// }

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("menu");
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![MouseButton::Left, MouseButton::Middle], // which buttons should drag the camera
        enabled: true, // when false, controls are disabled. See toggle example.
        zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
        min_scale: 1., // prevent the camera from zooming too far in
        max_scale: Some(40.), // prevent the camera from zooming too far out
        // Optinally specify min_x, max_x, min_y, max_y to limit the camera's movement
        ..Default::default()
    });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Open source",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.github.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
