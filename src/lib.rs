#![allow(clippy::type_complexity)]

const HELP_FONT_SIZE: f32 = 1.0;
const TEXT_COLOR: Color = Color::Rgba {
    red: 0.,
    green: 0.5,
    blue: 0.5,
    alpha: 1.,
};
const HELP_TEXT_PADDING: Val = Val::Px(15.0);

mod actions;
mod audio;
mod loading;
mod menu;
mod player;

mod camera;
mod bees;
mod boids;
mod debug;
mod world;
mod interactions;
mod tilemap;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::debug::debug::DebugPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use crate::bees::BeesPlugin;
use crate::world::WorldPlugin;
use crate::camera::CameraPlugin;
use crate::tilemap::MapPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::interactions::InteractionsPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // Additional state for pausing the game
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            // InternalAudioPlugin,
            // PlayerPlugin,
            CameraPlugin,
            MapPlugin,
            // WorldPlugin,
            InteractionsPlugin,
            BeesPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                DebugPlugin,
                // FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                WorldInspectorPlugin::new(),
            ))
            .add_systems(Startup, setup_entity_count)
            .add_systems(Update, update_entity_count);
        }
    }
}

#[derive(Component)]
struct EntityCount;

fn setup_entity_count(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: HELP_FONT_SIZE,
                color: TEXT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: HELP_TEXT_PADDING,
            right: HELP_TEXT_PADDING,
            ..default()
        }),
        EntityCount,
    ));
}

fn update_entity_count(entities: Query<Entity>, mut counters: Query<&mut Text, With<EntityCount>>) {
    let mut text = counters.single_mut();
    text.sections[0].value = format!("Entities: {:?}", entities.iter().len());
}
