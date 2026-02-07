//! The credits menu.

use bevy::{ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    asset_collection::AudioAssets, audio::music, camera::LevelPosition, menus::Menu,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

fn spawn_credits_menu(
    mut commands: Commands,
    mut camera: Single<&mut LevelPosition, With<Camera2d>>,
) {
    **camera = LevelPosition::new(0, -1);

    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Credits),
        children![
            widget::header("Created by"),
            created_by(),
            widget::header("Assets"),
            assets(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![["Galdormin", "Code & Design"], ["Nexia", "Art"]])
}

fn assets() -> impl Bundle {
    grid(vec![
        ["Ducky sprite", "CC0 by Caz Creates Games"],
        ["Button SFX", "CC0 by Jaszunio15"],
        ["Music", "CC BY 3.0 by Kevin MacLeod"],
        ["Bevy logo", "All rights reserved by the Bevy Foundation"],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(0),
            column_gap: px(20),
            grid_template_columns: RepeatedGridTrack::px(2, 200.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label(text),
                    Node {
                        justify_self: if i.is_multiple_of(2) {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn start_credits_music(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        DespawnOnExit(Menu::Credits),
        music(audio_assets.credit_music.clone()),
    ));
}
