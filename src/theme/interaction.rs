use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::{asset_collection::AudioAssets, audio::sound_effect};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(apply_interaction_palette_on_over);
    app.add_observer(apply_interaction_palette_on_out);

    app.add_observer(apply_selection_markers_on_over);
    app.add_observer(apply_selection_markers_on_out);

    app.add_observer(play_sound_effect_on_click);
    app.add_observer(play_sound_effect_on_over);
}

/// Change the [`TextColor`] based on the current [`Interaction`]` state.
/// The color is chosen randomly on over.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Vec<Color>,
}

/// Add `>` and `<` around the selected button when hovered.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SelectionMarkerText {
    /// Base text of the button to store.
    pub base: String,
}

fn apply_interaction_palette_on_over(
    over: On<Pointer<Over>>,
    mut palette_query: Query<(&InteractionPalette, &mut TextColor)>,
) {
    let Ok((palette, mut text)) = palette_query.get_mut(over.event_target()) else {
        return;
    };

    let rng = &mut rand::rng();
    *text = palette.hovered.choose(rng).unwrap().to_owned().into();
}

fn apply_interaction_palette_on_out(
    out: On<Pointer<Out>>,
    mut palette_query: Query<(&InteractionPalette, &mut TextColor)>,
) {
    let Ok((palette, mut text)) = palette_query.get_mut(out.event_target()) else {
        return;
    };

    *text = palette.none.into();
}

fn apply_selection_markers_on_over(
    over: On<Pointer<Over>>,
    mut marker_query: Query<(&SelectionMarkerText, &mut Text)>,
) {
    let Ok((marker, mut text)) = marker_query.get_mut(over.event_target()) else {
        return;
    };

    text.0 = format!("> {} <", marker.base);
}

fn apply_selection_markers_on_out(
    out: On<Pointer<Out>>,
    mut marker_query: Query<(&SelectionMarkerText, &mut Text)>,
) {
    let Ok((marker, mut text)) = marker_query.get_mut(out.event_target()) else {
        return;
    };

    text.0 = marker.base.clone();
}

fn play_sound_effect_on_click(
    _: On<Pointer<Click>>,
    audio_assets: If<Res<AudioAssets>>,
    mut commands: Commands,
) {
    commands.spawn(sound_effect(audio_assets.click_sound.clone()));
}

fn play_sound_effect_on_over(
    _: On<Pointer<Over>>,
    audio_assets: If<Res<AudioAssets>>,
    mut commands: Commands,
) {
    commands.spawn(sound_effect(audio_assets.hover_sound.clone()));
}
