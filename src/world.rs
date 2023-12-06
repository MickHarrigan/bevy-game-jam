use std::time::Duration;

use crate::bees::BoidGroup;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroupShaderType;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_pancam::PanCam;

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
            .insert_resource(QuadTreeVisualizer(false))
            .register_type::<QuadTreeVisualizer>()
            .add_systems(OnEnter(GameState::Playing), setup_level)
            .add_systems(Update, get_level_data.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                visualize_quadtree.run_if(in_state(GameState::Playing)),
            )
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

// fn cleanup_world(mut commands: Commands, world: Query<LdtkProject>) {
//     for entity in &mut world.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }

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

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct QuadTreeVisualizer(pub bool);

fn visualize_quadtree(mut gizmos: Gizmos, vis: Res<QuadTreeVisualizer>, groups: Query<&BoidGroup>) {
    if !vis.0 {
        return;
    }
    for group in groups.iter() {
        let regions = group.graph.get_regions();
        regions.iter().for_each(|reg| {
            let (min_x, min_y, max_x, max_y) = reg.into_f32();

            let bottom_left = Vec3::new(min_x, min_y, 0.0);
            let bottom_right = Vec3::new(max_x, min_y, 0.0);
            let top_right = Vec3::new(max_x, max_y, 0.0);
            let top_left = Vec3::new(min_x, max_y, 0.0);

            gizmos.line(bottom_left, bottom_right, Color::WHITE);
            gizmos.line(bottom_right, top_right, Color::WHITE);
            gizmos.line(top_right, top_left, Color::WHITE);
            gizmos.line(top_left, bottom_left, Color::WHITE);
        });
    }
}
