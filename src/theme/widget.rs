//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
};

use crate::{
    asset_collection::UiAssets,
    theme::{
        interaction::{InteractionPalette, SelectionMarkerText},
        palette::*,
    },
};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(24.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(16.0),
        TextColor(LABEL_TEXT),
    )
}

/// An Image node
pub fn image(image: Handle<Image>, width: Val) -> impl Bundle {
    (
        Name::new("Image"),
        Node {
            width,
            ..Default::default()
        },
        ImageNode::new(image),
    )
}

/// A horizontal space
pub fn hspace(height: Val) -> impl Bundle {
    (
        Name::new("Hspace"),
        Node {
            height,
            ..Default::default()
        },
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        true,
        Node {
            width: px(200),
            height: px(20),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        false,
        Node {
            width: px(20),
            height: px(20),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    with_markers: bool,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        button_bundle,
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            let mut text_entity = parent.spawn((
                Name::new("Button Text"),
                Text(text.clone()),
                TextFont::from_font_size(24.0),
                InteractionPalette {
                    none: BUTTON_TEXT,
                    hovered: BIOME_COLORS.into(),
                },
                TextColor(BUTTON_TEXT),
            ));
            if with_markers {
                text_entity.insert(SelectionMarkerText { base: text });
            }
            text_entity.observe(action);
        })),
    )
}

/// Add font family to button
pub(super) fn add_font_to_button(
    button_fonts: Query<&mut TextFont, Added<TextFont>>,
    ui_assets: Res<UiAssets>,
) {
    for mut font in button_fonts {
        font.font = ui_assets.main_font.clone();
    }
}
