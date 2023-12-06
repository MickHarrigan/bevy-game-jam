use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_pancam::PanCam;

use crate::menu::NextLevel;

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
            .add_systems(Update, get_level_data.run_if(in_state(GameState::Playing)))
            // .add_systems(OnExit(GameState::Playing), cleanup_world)
            .add_plugins(LdtkPlugin)
            // .add_plugins(PanCamPlugin::default());
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
    mut camera: Query<(&mut Transform, &mut OrthographicProjection, &mut PanCam), With<Camera2d>>,
) {
    // Change camera settings on playing state
    info!("Change camera settings on playing state");
    for (mut transform, mut projection, mut pancam) in &mut camera {
        // Set world camera scale and location
        projection.scale = 0.25;
        transform.translation.x = 30.0;
        transform.translation.y = 30.0;
        // Enable pancam plugin
        pancam.enabled = true;
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

fn get_level_data(
    level: Res<Assets<LdtkProject>>,
    mut camera: Query<&mut PanCam, With<Camera2d>>,
    handle: Res<LdtkLevel>,
    mut loaded: Local<bool>,
) {
    // get the level of a handle
    if *loaded {
        return;
    }

    let mut pancam = camera.single_mut();
    if let Some(data) = level.get(&handle.0) {
        let height = data.iter_root_levels().next().unwrap().px_hei;
        let width = data.iter_root_levels().next().unwrap().px_wid;
        pancam.min_scale = 0.1;
        pancam.max_scale = Some(100.);
        pancam.max_x = Some(width as f32);
        pancam.max_y = Some(height as f32);
        pancam.min_x = Some(0.);
        pancam.min_y = Some(0.);
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
