use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::loading::TextureAssets;

use rand::Rng;

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
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    let texture_handle: Handle<Image> = textures.ground.clone();

    let map_size = TilemapSize { x: 16, y: 16 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);
    let mut rng = rand::thread_rng();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos{ x , y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(rng.gen_range(0..3)),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize{ x: 720.0, y: 720.0 };
    let grid_size: TilemapGridSize = tile_size.into();
    commands.insert_resource(LevelData {level_height: grid_size.y, level_width: grid_size.x});
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
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

}