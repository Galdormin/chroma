//! Load asset through Bevy Asset Loader

use bevy::prelude::*;

use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
use iyes_progress::ProgressPlugin;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ProgressPlugin::<Screen>::new());
    app.add_loading_state(
        LoadingState::new(Screen::Splash)
            .load_collection::<AudioAssets>()
            .load_collection::<LevelAssets>()
            .load_collection::<PlayerAssets>()
            .load_collection::<UiAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "images/ducky.png")]
    pub ducky_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 6,
        rows = 2,
        padding_x = 1,
        padding_y = 1,
        offset_x = 0,
        offset_y = 0
    ))]
    pub ducky_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // Musics
    #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
    pub credit_music: Handle<AudioSource>,
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    pub game_music: Handle<AudioSource>,

    // SFX
    #[asset(
        paths(
            "audio/sound_effects/step1.ogg",
            "audio/sound_effects/step2.ogg",
            "audio/sound_effects/step3.ogg",
            "audio/sound_effects/step4.ogg",
        ),
        collection(typed)
    )]
    pub step_sfx: Vec<Handle<AudioSource>>,

    // Ui Sounds
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub hover_sound: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub click_sound: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    // Images
    #[asset(path = "images/chroma_title.png")]
    pub title_art: Handle<Image>,

    // Fonts
    #[asset(path = "fonts/monogram.ttf")]
    pub main_font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "world.ldtk")]
    pub world: Handle<LdtkProject>,
}
