use std::time::Duration;

use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroupShaderType;
use bevy_ecs_ldtk::prelude::*;
// use bevy_pancam::PanCam;

use crate::loading::TextureAssets;
use crate::menu::NextLevel;

// use bevy_pancam::{PanCam, PanCamPlugin};

pub struct WorldPlugin;

// This plugin is responsible for spawning the LDtk game world and entities
// The world is only spawned during the State `GameState::Playing` and is removed when that state is exited

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // LDtk level selection resource
            .insert_resource(LevelSelection::index(0))
            .init_resource::<LdtkLevel>()
            .add_systems(OnEnter(GameState::Playing), setup_level)
            .insert_resource(LevelData { level_height: 0, level_width: 0 })
            .add_systems(Update, get_level_data.run_if(in_state(GameState::Playing)))
            .add_plugins(LdtkPlugin)
            // Register LDtk entities
            .register_ldtk_entity::<QueenBundle>("Queen")
            .register_ldtk_entity::<EnemyQueenBundle>("EnemyQueen");
    }
}

#[derive(Resource, Default)]
pub struct LdtkLevel(pub Handle<LdtkProject>);

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<NextLevel>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
) {
    // Change camera settings on playing state
    info!("Change camera settings on playing state");
    for (mut transform, mut projection) in &mut camera {
        // Set world camera scale and location
        projection.scale = 0.25;
        transform.translation.x = 30.0;
        transform.translation.y = 30.0;
    }

    let level_handle = asset_server.load(level.0);

    commands.insert_resource(LdtkLevel(level_handle.clone()));

    // Spawn LDTK level
    info!("Spawn LDTK level");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_handle,
        ..Default::default()
    });
}

#[derive(Resource, Debug)]
pub struct LevelData {
    pub level_height: i32,
    pub level_width: i32,
}

fn get_level_data(
    level: Res<Assets<LdtkProject>>,
    // mut camera: Query<Camera>,
    handle: Res<LdtkLevel>,
    mut loaded: Local<bool>,
    mut level_data: ResMut<LevelData>,
) {
    // get the level of a handle
    if *loaded {
        return;
    }

    // let mut pancam = camera.single_mut();
    if let Some(data) = level.get(&handle.0) {
        level_data.level_height = data.iter_root_levels().next().unwrap().px_hei;
        level_data.level_width = data.iter_root_levels().next().unwrap().px_wid;

        *loaded = true;
    }
}

#[derive(Default, Component)]
pub struct Queen;
// Spawning sprites for LDtk entities
#[derive(Default, Bundle, LdtkEntity)]
struct QueenBundle {
    queen: Queen,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct EnemyQueenBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
