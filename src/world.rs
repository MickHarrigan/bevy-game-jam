use std::time::Duration;

use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_pancam::PanCam;

use crate::loading::TextureAssets;

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
            .add_plugins(LdtkPlugin)
            // .add_plugins(PanCamPlugin::default());
            // Register LDtk entities
            .register_ldtk_entity::<QueenBundle>("Queen")
            .register_ldtk_entity::<EnemyQueenBundle>("EnemyQueen")
            .register_ldtk_entity::<TowerBundle>("Tower")
            .add_systems(OnEnter(GameState::Playing), setup_tower_shooting)
            .add_systems(Update, tower_shoot.run_if(in_state(GameState::Playing)))
            .add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)));
    }
}

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

// Spawning sprites for LDtk entities
#[derive(Default, Bundle, LdtkEntity)]
struct QueenBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct EnemyQueenBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Component)]
struct Tower;

#[derive(Default, Bundle, LdtkEntity)]
struct TowerBundle {
    tower: Tower,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Component)]
struct Bullet;

#[derive(Resource)]
struct TowerShootTimer {
    // How often to spawn a bullet (repeating timer)
    timer: Timer,
}

// Shoot bullets from towers
fn tower_shoot(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<&Transform, With<Tower>>,
    mut bullet_timer: ResMut<TowerShootTimer>,
    textures: Res<TextureAssets>,
) {
    bullet_timer.timer.tick(time.delta());

    if bullet_timer.timer.finished() {
        for transform in &mut query.iter() {
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: textures.shmup.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        3.0,
                    ),
                    ..default()
                },
                Bullet,
            ));
        }
    }
}

// Enable tower shooting
fn setup_tower_shooting(mut commands: Commands) {
    commands.insert_resource(TowerShootTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
    });
}

// Move bullets
fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    // Despawn Y level
    let despawn_y_level = 300.0;

    for (entity, mut transform) in bullet.iter_mut() {
        transform.translation.y += time.delta_seconds() * 30.0;

        if transform.translation.y > despawn_y_level {
            commands.entity(entity).despawn();
            info!("Despawn bullet")
        }
    }
}

