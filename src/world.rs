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
            .add_systems(Update, get_level_data.run_if(in_state(GameState::Playing)))
            // .add_systems(OnExit(GameState::Playing), cleanup_world)
            .add_plugins(LdtkPlugin)
            // .add_plugins(PanCamPlugin::default());
            // Register LDtk entities
            .register_ldtk_entity::<QueenBundle>("Queen")
            .register_ldtk_entity::<EnemyQueenBundle>("EnemyQueen");
        // .register_ldtk_entity::<TowerBundle>("Tower")
        // .add_systems(OnEnter(GameState::Playing), setup_tower_shooting)
        // .add_systems(Update, tower_shoot.run_if(in_state(GameState::Playing)))
        // .add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)));
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

fn get_level_data(
    level: Res<Assets<LdtkProject>>,
    // mut camera: Query<Camera>,
    handle: Res<LdtkLevel>,
    mut loaded: Local<bool>,
) {
    // get the level of a handle
    if *loaded {
        return;
    }

    // let mut pancam = camera.single_mut();
    if let Some(data) = level.get(&handle.0) {
        // let height = data.iter_root_levels().next().unwrap().px_hei;
        // let width = data.iter_root_levels().next().unwrap().px_wid;
        // pancam.min_scale = 0.1;
        // pancam.max_scale = Some(100.);
        // pancam.max_x = Some(width as f32);
        // pancam.max_y = Some(height as f32);
        // pancam.min_x = Some(0.);
        // pancam.min_y = Some(0.);
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

// #[derive(Default, Component)]
// struct Tower;

// #[derive(Default, Bundle, LdtkEntity)]
// struct TowerBundle {
//     tower: Tower,
//     #[sprite_sheet_bundle]
//     sprite_sheet_bundle: SpriteSheetBundle,
// }

// #[derive(Default, Component)]
// struct Bullet;

// #[derive(Resource)]
// struct TowerShootTimer {
//     // How often to spawn a bullet (repeating timer)
//     timer: Timer,
// }

// // Shoot bullets from towers
// fn tower_shoot(
//     mut commands: Commands,
//     time: Res<Time>,
//     query: Query<&Transform, With<Tower>>,
//     mut bullet_timer: ResMut<TowerShootTimer>,
//     textures: Res<TextureAssets>,
// ) {
//     bullet_timer.timer.tick(time.delta());

//     if bullet_timer.timer.finished() {
//         for transform in &mut query.iter() {
//             commands.spawn((
//                 SpriteSheetBundle {
//                     texture_atlas: textures.shmup.clone(),
//                     sprite: TextureAtlasSprite::new(0),
//                     transform: Transform::from_xyz(
//                         transform.translation.x,
//                         transform.translation.y,
//                         3.0,
//                     ),
//                     ..default()
//                 },
//                 Bullet,
//             ));
//         }
//     }
// }

// // Enable tower shooting
// fn setup_tower_shooting(mut commands: Commands) {
//     commands.insert_resource(TowerShootTimer {
//         timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
//     });
// }

// // Move bullets
// fn bullet_movement(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut bullet: Query<(Entity, &mut Transform), With<Bullet>>,
// ) {
//     // Despawn Y level
//     let despawn_y_level = 300.0;

//     for (entity, mut transform) in bullet.iter_mut() {
//         transform.translation.y += time.delta_seconds() * 30.0;

//         if transform.translation.y > despawn_y_level {
//             commands.entity(entity).despawn();
//             info!("Despawn bullet")
//         }
//     }
// }
