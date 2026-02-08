//! The title screen that appears after the splash screen.

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkProjectHandle, LdtkWorldBundle};

use crate::{
    asset_collection::{AudioAssets, LevelAssets},
    audio::{AudioSettings, music},
    menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), (setup_world, open_main_menu));
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn setup_world(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
    world: Query<Entity, With<LdtkProjectHandle>>,
) {
    if !world.is_empty() {
        return;
    }

    commands.spawn((
        Name::new("Level"),
        LdtkWorldBundle {
            ldtk_handle: level_assets.world.clone().into(),
            ..Default::default()
        },
    ));

    commands.spawn((
        Name::new("Main Music"),
        music(audio_assets.main_music.clone(), &audio_settings),
    ));
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
