//! All Entities of the game world

use bevy::prelude::*;

pub mod droplet;
pub mod object;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((object::plugin, droplet::plugin));
}
