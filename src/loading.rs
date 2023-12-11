use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_asset_loader::asset_collection::AssetCollection;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        // .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,

    // #[asset(texture_atlas(
    //     tile_size_x = 16.,
    //     tile_size_y = 16.,
    //     columns = 12,
    //     rows = 10,
    //     padding_x = 0.,
    //     padding_y = 0.,
    //     offset_x = 0.,
    //     offset_y = 0.
    // ))]
    // #[asset(path = "textures/shmup.png")]
    // pub shmup: Handle<TextureAtlas>,
    //
    // #[asset(texture_atlas(
    //     tile_size_x = 32.,
    //     tile_size_y = 32.,
    //     columns = 4,
    //     rows = 6,
    //     padding_x = 0.,
    //     padding_y = 0.,
    //     offset_x = 0.,
    //     offset_y = 0.
    // ))]
    // #[asset(path = "textures/airplanes.png")]
    // pub planes: Handle<TextureAtlas>,

    #[asset(path = "textures/menubackground.png")]
    pub background: Handle<Image>,

    // #[asset(texture_atlas(tile_size_x = 720., tile_size_y = 720., columns = 4, rows = 1, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "textures/ground/ground.png")]
    pub ground: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 720., tile_size_y = 720., columns = 3, rows = 1, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "textures/bee1/wingmap1.png")]
    pub bee1wingmap: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 720., tile_size_y = 720., columns = 3, rows = 1, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "textures/bee2/wingmap2.png")]
    pub bee2wingmap: Handle<TextureAtlas>,

    #[asset(path = "textures/bee1/beebody.png")]
    pub beebody1: Handle<Image>,

    #[asset(path = "textures/bee2/beebody.png")]
    pub beebody2: Handle<Image>,

    #[asset(path = "textures/queen.png")]
    pub queen: Handle<Image>,

    #[asset(path = "textures/hive.png")]
    pub hive: Handle<Image>,

    #[asset(path = "textures/fog.png")]
    pub fog: Handle<Image>,

    #[asset(path = "textures/flowers/flower1.png")]
    pub flower1: Handle<Image>,

    #[asset(path = "textures/flowers/flower2.png")]
    pub flower2: Handle<Image>,

    #[asset(path = "textures/flowers/flower3.png")]
    pub flower3: Handle<Image>,

    #[asset(path = "textures/flowers/flower4.png")]
    pub flower4: Handle<Image>,
}
