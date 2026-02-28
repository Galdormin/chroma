use bevy::prelude::*;

use crate::{
    ldtk::{GameColor, Tint},
    player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_player_color);
}

#[derive(Bundle)]
pub struct CharacterVisualBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    tint: Tint,
}

impl CharacterVisualBundle {
    pub fn new(
        shape: Capsule2d,
        color: GameColor,
        mut mesh_assets: ResMut<Assets<Mesh>>,
        mut material_asets: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            mesh: Mesh2d(mesh_assets.add(shape)),
            material: MeshMaterial2d(material_asets.add(color.color())),
            tint: color.into(),
        }
    }
}

fn update_player_color(
    players: Query<(&mut MeshMaterial2d<ColorMaterial>, &Tint), (Changed<Tint>, With<Player>)>,
    mut material_asets: ResMut<Assets<ColorMaterial>>,
) {
    for (mut material, tint) in players {
        let game_color = match tint.get_colors()[..] {
            [color] => color,
            [] => {
                warn!("Player should have a color. Use default to GameColor::White");
                GameColor::White
            }
            [color, ..] => {
                warn!("Player should have a single color.");
                color
            }
        };

        *material = MeshMaterial2d(material_asets.add(game_color.color()))
    }
}
