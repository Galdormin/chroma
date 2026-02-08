//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}

use bevy::prelude::*;

use crate::asset_collection::UiAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin).add_systems(
        Update,
        widget::add_font_to_button.run_if(resource_exists::<UiAssets>),
    );
}
