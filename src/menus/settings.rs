//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    audio::{AudioSettings, VolumeType},
    camera::LevelPosition,
    menus::Menu,
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(Update, update_volume_label.run_if(in_state(Menu::Settings)));
}

fn spawn_settings_menu(
    mut commands: Commands,
    mut camera: Single<&mut LevelPosition, With<Camera2d>>,
) {
    **camera = LevelPosition::new(0, 1);

    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            volume_label(VolumeType::Master),
            volume_widget(VolumeType::Master),
            volume_label(VolumeType::Music),
            volume_widget(VolumeType::Music),
            volume_label(VolumeType::Sfx),
            volume_widget(VolumeType::Sfx),
        ],
    )
}

fn volume_label(volume_type: VolumeType) -> impl Bundle {
    (
        widget::label(format!("{} Volume", volume_type.to_string())),
        Node {
            justify_self: JustifySelf::End,
            ..default()
        },
    )
}

fn volume_widget(volume_type: VolumeType) -> impl Bundle {
    (
        Name::new("Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_volume(volume_type)),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), VolumeLabel(volume_type))],
            ),
            widget::button_small("+", raise_volume(volume_type)),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_volume(volume_type: VolumeType) -> impl Fn(On<Pointer<Click>>, ResMut<AudioSettings>) {
    move |_: On<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>| {
        let volume = (audio_settings.get_volume(volume_type) - 0.1).max(MIN_VOLUME);
        audio_settings.set_volume(volume_type, volume);
    }
}

fn raise_volume(volume_type: VolumeType) -> impl Fn(On<Pointer<Click>>, ResMut<AudioSettings>) {
    move |_: On<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>| {
        let volume = (audio_settings.get_volume(volume_type) + 0.1).min(MAX_VOLUME);
        audio_settings.set_volume(volume_type, volume);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct VolumeLabel(VolumeType);

fn update_volume_label(
    audio_settings: Res<AudioSettings>,
    labels: Query<(&mut Text, &VolumeLabel)>,
) {
    for (mut text, volume_label) in labels {
        let percent = 100.0 * audio_settings.get_volume(volume_label.0);
        text.0 = format!("{percent:3.0}%");
    }
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
