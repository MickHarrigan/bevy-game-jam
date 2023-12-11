use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::loading::TextureAssets;

use rand::Rng;
use crate::bees::{Bee, BeeBehavior, Collider, Velocity};
use crate::boids::Boid;
use crate::interactions::Highlightable;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_systems(OnEnter(GameState::Playing), setup_level)
        ;
    }
}

#[derive(Resource, Debug)]
pub struct LevelData {
    pub level_height: f32,
    pub level_width: f32,
}

fn setup_level(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    textures: Res<TextureAssets>,
    mut q_camera: Query<&mut Transform, With<Camera2d>>,

    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    let texture_handle: Handle<Image> = textures.ground.clone();

    let map_size = TilemapSize { x: 32, y: 32 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);
    let mut rng = rand::thread_rng();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(rng.gen_range(0..7)),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 720.0, y: 720.0 };
    let grid_size: TilemapGridSize = tile_size.into();
    commands.insert_resource(LevelData { level_height: grid_size.y * map_size.y as f32, level_width: grid_size.x * map_size.x as f32 });
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        transform: Transform::from_xyz(360., 360., 0.),
        ..Default::default()
    });

    // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // Only used when the atlas feature is off and we are using array textures.
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("tiles.png")),
            tile_size,
            ..Default::default()
        });
    }
    // Spawn fog
    // Create a separate tilemap for fog
    let fog_tilemap_entity = commands.spawn_empty().id();
    let mut fog_tile_storage = TileStorage::empty(map_size);

    // Populate the fog tile storage with fog tiles covering the entire map
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let fog_tile_pos = TilePos { x, y };
            let fog_tile_entity = commands
                .spawn(TileBundle {
                    position: fog_tile_pos,
                    tilemap_id: TilemapId(fog_tilemap_entity),
                    texture_index: TileTextureIndex(0), // Set fog texture index
                    ..Default::default()
                })
                .id();
            fog_tile_storage.set(&fog_tile_pos, fog_tile_entity);
        }
    }

    // Insert fog tilemap bundle
    commands.entity(fog_tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: fog_tile_storage.clone(),
            texture: TilemapTexture::Single(textures.fog.clone()), // Use fog texture handle
            tile_size,
            transform: Transform::from_translation(Vec3::new(360.0, 360.0, 1.0)), // Render fog on top by adjusting z-axis
            ..Default::default()

        },
        FogTile
    ));


    // Spawn the hive
    let max_x = (map_size.x * tile_size.x as u32) - 720;
    let max_y = (map_size.y * tile_size.y as u32) - 720;

    let beehive_position = Vec3::new(
        ((rand::random::<u32>() % max_x) as f32 / tile_size.x * tile_size.x + 360.) as f32,
        ((rand::random::<u32>() % max_y) as f32 / tile_size.y * tile_size.y + 360.) as f32,
        2.0,
    );

    commands.spawn(SpriteBundle {
        texture: textures.hive.clone(),
        transform: Transform {
            translation: beehive_position,
            ..Default::default()
        },
        ..Default::default()
    });
    info!("Spawned beehive at {:?}", beehive_position);
    let mut camera_transform = q_camera.single_mut();
    camera_transform.translation = beehive_position;

    // Despawn fog around hive
    // Calculate fog removal area around the beehive
    let fog_removal_radius = 3; // Adjust the radius as needed
    // Ensure fog removal area stays within the bounds of the tile storage
    let fog_tile_storage_size = fog_tile_storage.size; // Assuming fog_tile_storage has a size() method
    let beehive_tile_pos = TilePos {
        x: (beehive_position.x / tile_size.x) as u32,
        y: (beehive_position.y / tile_size.y) as u32,
    };
    let mut fog_removal_area = Vec::new();
    for x in (beehive_tile_pos.x as i32 - fog_removal_radius as i32)..=(beehive_tile_pos.x as i32 + fog_removal_radius as i32) {
        for y in (beehive_tile_pos.y as i32 - fog_removal_radius as i32)..=(beehive_tile_pos.y as i32 + fog_removal_radius as i32) {
            if x >= 0 && y >= 0 && x < fog_tile_storage_size.x as i32 && y < fog_tile_storage_size.y as i32 {
                fog_removal_area.push((x as u32, y as u32));
            }
        }
    }
    // Access the fog tile storage and remove fog tiles in the calculated area
    for (x, y) in fog_removal_area {
        if let Some(fog_tile_entity) = fog_tile_storage.get(&TilePos { x, y }) {
            commands.entity(fog_tile_entity).despawn_recursive();
        }
    }

    // Spawn the Queen bee
    // Calculate the maximum position bounds
    let max_x = (map_size.x * tile_size.x as u32) - 720;
    let max_y = (map_size.y * tile_size.y as u32) - 720;

    // Spawn the bee queen away from the hive
    let bee_queen_position = get_random_position_away_from_hive(beehive_position, max_x, max_y, grid_size, 720);
    commands.spawn(SpriteBundle {
        texture: textures.queen.clone(),
        transform: Transform {
            translation: bee_queen_position,
            ..Default::default()
        },
        ..Default::default()
    });
    info!("Spawned bee queen at {:?}", bee_queen_position);

    // Spawn flowers randomly within the map bounds
    const NUM_FLOWERS: usize = 20;
    for _ in 0..NUM_FLOWERS {
        let flower_type = rand::random::<u8>() % 4; // Assuming you have 4 flower types
        let flower_texture = match flower_type {
            0 => textures.flower1.clone(),
            1 => textures.flower2.clone(),
            2 => textures.flower3.clone(),
            3 => textures.flower4.clone(),
            _ => textures.flower1.clone(), // Default to a texture if needed
        };

        let flower_position = get_random_position(max_x, max_y, grid_size);

        commands.spawn((
            SpriteBundle {
                texture: flower_texture,
                transform: Transform {
                    translation: flower_position,
                    ..Default::default()
                },
                ..Default::default()
            },
            Flower
        ));
    }

    // Spawn the initial 5 bees
    for _ in 0..5 {
        let bee_body = if rng.gen::<bool>() {
            textures.beebody1.clone()
        } else {
            textures.beebody2.clone()
        };
        let bee_offset_x = rng.gen_range(-500.0..=500.0); // Adjust the offset range as needed
        let bee_offset_y = rng.gen_range(-500.0..=500.0);
        let bee_entity = commands.spawn((
            SpriteBundle {
                texture: bee_body,
                transform:  Transform::from_translation(Vec3::new(
                    beehive_position.x + bee_offset_x,
                    beehive_position.y + bee_offset_y,
                    beehive_position.z,
                )),
                ..default()
            },
            Bee,
            BeeBehavior::Destination(Vec2::new(beehive_position.x, beehive_position.y)),
            Boid,
            Highlightable,
            Collider::new(25.0),
            Velocity::default(),
        )).id();

        // Spawn the wings as a child of the bee body
        let bee_wings= if rng.gen::<bool>() {
            textures.bee1wingmap.clone()
        } else {
            textures.bee2wingmap.clone()
        };
        commands.entity(bee_entity).with_children(|parent| {
            parent.spawn(SpriteSheetBundle {
                texture_atlas: bee_wings,
                sprite: TextureAtlasSprite::new(0), // Set the initial sprite index
                transform: Transform::from_xyz(0.0, 0.0, 1.0), // Adjust the position of the wings relative to the bee body
                ..Default::default()
            })
                .insert(crate::bees::AnimationIndices { first: 0, last: 3 - 1 })
                .insert(crate::bees::AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)));
        });

        // info!("Bee spawned at hive position");
    }

}

// Define a function to get a random position away from the hive
fn get_random_position_away_from_hive(hive_position: Vec3, max_x: u32, max_y: u32, tile_size: TilemapGridSize, radius: u32) -> Vec3 {
    loop {
        let x = (rand::random::<u32>() % max_x) as f32 / tile_size.x * tile_size.x + 360.;
        let y = (rand::random::<u32>() % max_y) as f32 / tile_size.y * tile_size.y + 360.;
        let candidate_position = Vec3::new(x, y, 1.0);

        // Check if the candidate position is away from the hive by the given radius
        if (candidate_position - hive_position).length() > radius as f32 {
            return candidate_position;
        }
    }
}

// Define a function to get a random position within the map bounds
fn get_random_position(max_x: u32, max_y: u32, tile_size: TilemapGridSize) -> Vec3 {
    let x = (rand::random::<u32>() % max_x) as f32 / tile_size.x * tile_size.x + 360.;
    let y = (rand::random::<u32>() % max_y) as f32 / tile_size.y * tile_size.y + 360.;
    Vec3::new(x, y, 1.0)
}

#[derive(Component)]
pub struct Flower;

#[derive(Component)]
pub struct FogTile;
