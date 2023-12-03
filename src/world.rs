use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::GameState;

// use bevy_pancam::{PanCam, PanCamPlugin};

pub struct WorldPlugin;

// This plugin is responsible for spawning the LDtk game world and entities
// The world is only spawned during the State `GameState::Playing` and is removed when that state is exited

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // LDtk level selection resource
            .insert_resource(LevelSelection::index(0))
            .add_systems(OnEnter(GameState::Playing), setup_level)
            // .add_systems(OnExit(GameState::Playing), cleanup_world)
            .add_plugins(LdtkPlugin);
            // .add_plugins(PanCamPlugin::default());
    }
}

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
) {
    // Change camera settings on playing state
    info!("Change camera settings on playing state");
    for (mut transform, mut projection) in &mut camera {
        projection.scale = 0.5;
        transform.translation.x += 1280.0 / 4.0;
        transform.translation.y += 720.0 / 4.0;
    }

    // Spawn LDTK level
    info!("Spawn LDTK level");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("world.ldtk"),
        ..Default::default()
    });
}

// fn cleanup_world(mut commands: Commands, world: Query<LdtkProject>) {
//     for entity in &mut world.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }