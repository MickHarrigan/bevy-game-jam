use std::time::Duration;

use crate::GameState;
use bevy::{prelude::*, utils::tracing::field::debug};
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
            .register_ldtk_int_cell::<DirtBundle>(1)
            .register_ldtk_int_cell::<GrassBundle>(2)
            .add_systems(OnEnter(GameState::Playing), setup_tower_shooting)
            .add_systems(OnEnter(GameState::Playing), setup_spawning)
            .add_systems(Update, tower_shoot.run_if(in_state(GameState::Playing)))
            .add_systems(Update, debug_find.run_if(in_state(GameState::Playing)))
            .add_systems(Update, spawn_swarm.run_if(in_state(GameState::Playing)))
            .add_systems(Update, move_swarmer.run_if(in_state(GameState::Playing)))
            .add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Default, Component)]
struct Dirt;

#[derive(Default, Bundle, LdtkIntCell)]
struct DirtBundle {
    dirt: Dirt,
}

#[derive(Default, Component)]
struct Grass;

#[derive(Default, Bundle, LdtkIntCell)]
struct GrassBundle {
    dirt: Grass,
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

#[derive(Component, Default)]
struct Spawner;

#[derive(Component, Default)]
struct Team(u32);

impl From<&EntityInstance> for Team {
    fn from(entity_instance: &EntityInstance) -> Team {
        for inst in entity_instance.field_instances.iter() {
            if inst.identifier == "Team".to_string() {
                if let FieldValue::Int(a) = inst.value {
                    return Team(a.unwrap() as u32);
                }
            }
        }
        Team(0)
    }
}

// Spawning sprites for LDtk entities
#[derive(Default, Bundle, LdtkEntity)]
struct QueenBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    spawner: Spawner,
    #[from_entity_instance]
    team: Team,
}

#[derive(Default, Bundle, LdtkEntity)]
struct EnemyQueenBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    spawner: Spawner,
    #[from_entity_instance]
    team: Team,
}

#[derive(Default, Component)]
struct Tower;

#[derive(Default, Bundle, LdtkEntity)]
struct TowerBundle {
    tower: Tower,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    team: Team,
}

#[derive(Default, Component)]
struct Bullet;

#[derive(Resource)]
struct TowerShootTimer {
    // How often to spawn a bullet (repeating timer)
    timer: Timer,
}

#[derive(Resource)]
struct SpawnerTimer {
    // How often to spawn a Swarmer (repeating timer)
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

fn setup_spawning(mut commands: Commands) {
    commands.insert_resource(SpawnerTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
    });
}

fn spawn_swarm(
    queens: Query<(&Transform, &Team), With<Spawner>>,
    mut commands: Commands,
    time: Res<Time>,
    mut spawner_timer: ResMut<SpawnerTimer>,
    textures: Res<TextureAssets>,
) {
    // spawn in other entities that move to the other queen then die
    // they should also have a team applied to them

    spawner_timer.timer.tick(time.delta());

    if spawner_timer.timer.finished() {
        for (transform, team) in &mut queens.iter() {
            if team.0 == 0 {
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
                    Swarmer,
                    Team(team.0),
                ));
            }
        }
    }
}

#[derive(Default, Component)]
struct Swarmer;

fn move_swarmer(
    mut commands: Commands,
    time: Res<Time>,
    mut swarmers: Query<(Entity, &mut Transform, &Team), (With<Swarmer>, Without<Spawner>)>,
    queens: Query<(&Transform, &Team), (With<Spawner>, Without<Swarmer>)>,
) {
    // update the position on the swarmer, and if it reaches the enemy queen, then die
    for (queen_location, queen_team) in queens.iter() {
        for (entity, mut swarmer_loc, swarmer_team) in swarmers.iter_mut() {
            if swarmer_team.0 != queen_team.0 {
                let direction = queen_location.translation - swarmer_loc.translation;
                if direction.length() < 0.001 {
                    commands.entity(entity).despawn_recursive();
                    info!("Despawned a swarmer @ {}", direction.length_squared());
                } else {
                    swarmer_loc.translation += direction.normalize() * 30.0 * time.delta_seconds();
                }
            }
        }
    }
}

fn debug_find(tags: Query<Entity, With<Spawner>>, world: &World) {
    // for tag in &tags {
    //     info!("---------------------------------------------------------------------------");
    //     // info!("Components: {:#?}", world.inspect_entity(tag));
    //     info!("---------------------------------------------------------------------------");
    // }
}
