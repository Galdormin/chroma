use bevy::prelude::*;

use crate::{ldtk::wall::Tint, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, change_player_tint);

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
        tint: Tint,
        mut mesh_assets: ResMut<Assets<Mesh>>,
        mut material_asets: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            mesh: Mesh2d(mesh_assets.add(shape)),
            material: MeshMaterial2d(material_asets.add(tint.color())),
            tint,
        }
    }
}

fn update_player_color(
    players: Query<(&mut MeshMaterial2d<ColorMaterial>, &Tint), (Changed<Tint>, With<Player>)>,
    mut material_asets: ResMut<Assets<ColorMaterial>>,
) {
    for (mut material, tint) in players {
        *material = MeshMaterial2d(material_asets.add(tint.color()))
    }
}

fn change_player_tint(players: Query<&mut Tint, With<Player>>, input: Res<ButtonInput<KeyCode>>) {
    if !input.just_pressed(KeyCode::Tab) {
        return;
    }

    for mut tint in players {
        *tint = match *tint {
            Tint::White => Tint::Brown,
            Tint::Grey => Tint::White,
            Tint::Green => Tint::Grey,
            Tint::Brown => Tint::Green,
        };
    }
}
