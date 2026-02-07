//! The title screen that appears after the splash screen.

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

use crate::{asset_collection::LevelAssets, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), (setup_world, open_main_menu));
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn setup_world(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        LdtkWorldBundle {
            ldtk_handle: level_assets.world.clone().into(),
            ..Default::default()
        },
    ));
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
