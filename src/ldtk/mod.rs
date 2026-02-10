//! Module for LDtk bundles

use bevy::prelude::*;

pub mod entities;
pub mod wall;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((entities::plugin, wall::plugin));
}
