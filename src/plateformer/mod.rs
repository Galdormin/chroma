//!

use bevy::prelude::*;

pub mod entities;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(entities::plugin);
}
