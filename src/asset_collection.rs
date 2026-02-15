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
pub struct PlayerAssets {}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // Musics
    #[asset(path = "audio/music/exploration.ogg")]
    pub main_music: Handle<AudioSource>,

    // Ui Sounds
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub hover_sound: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub click_sound: Handle<AudioSource>,

    // SFX Sounds
    #[asset(path = "audio/sound_effects/paint_spray.ogg")]
    pub paint_spray: Handle<AudioSource>,
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
