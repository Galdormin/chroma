//! Module for LDtk bundles

use std::str::FromStr;

use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{prelude::LdtkFields, *};

use crate::theme::palette::*;

pub mod entities;
pub mod gridvania;
pub mod wall;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((entities::plugin, gridvania::plugin, wall::plugin));
}

/// Describe all the color of a game object
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Tint(HashSet<GameColor>);

impl Tint {
    pub fn from_color(color: GameColor) -> Self {
        color.into()
    }

    /// Return a tint based on the colors field, i.e. an array of Enum.
    pub fn from_colors_field(instance: &EntityInstance) -> Self {
        instance
            .get_maybe_enums_field("colors")
            .unwrap()
            .iter()
            .filter_map(|color| {
                color
                    .as_ref()
                    .and_then(|color_str| color_str.parse::<GameColor>().ok())
            })
            .collect::<_>()
    }

    /// Return a tint based on the `color` field, i.e. a single Enum.
    #[allow(dead_code)]
    pub fn from_color_field(instance: &EntityInstance) -> Self {
        instance
            .get_maybe_enum_field("color")
            .unwrap()
            .as_ref()
            .and_then(|c| c.parse::<GameColor>().ok())
            .into()
    }

    /// Return a vector with all the color
    pub fn get_colors(&self) -> Vec<GameColor> {
        self.0.iter().copied().collect()
    }

    /// Return True if the Tint as the given color
    #[allow(dead_code)]
    pub fn has_color(&self, color: GameColor) -> bool {
        self.0.contains(&color)
    }

    /// Return True if it share a color with the other Tint
    pub fn share_color_with(&self, other: &Tint) -> bool {
        self.0.difference(&other.0).count() < self.0.len()
    }
}

impl From<Option<GameColor>> for Tint {
    fn from(maybe_color: Option<GameColor>) -> Self {
        match maybe_color {
            Some(color) => color.into(),
            None => Self(HashSet::new()),
        }
    }
}

impl From<GameColor> for Tint {
    fn from(color: GameColor) -> Self {
        Self(HashSet::from([color]))
    }
}

impl From<Vec<GameColor>> for Tint {
    fn from(colors: Vec<GameColor>) -> Self {
        Self::from_iter(colors)
    }
}

impl FromIterator<GameColor> for Tint {
    fn from_iter<T: IntoIterator<Item = GameColor>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<HashSet<_>>())
    }
}

/// List of Color for game elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Hash)]
pub enum GameColor {
    White,
    Grey,
    Green,
    Brown,
    Orange,
}

impl FromStr for GameColor {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "White" => Ok(Self::White),
            "Grey" => Ok(Self::Grey),
            "Green" => Ok(Self::Green),
            "Brown" => Ok(Self::Brown),
            "Orange" => Ok(Self::Orange),
            _ => Err(format!("Cannot parse {s} as GameColor.")),
        }
    }
}

impl GameColor {
    pub fn color(&self) -> Color {
        match self {
            Self::White => WHITE,
            Self::Grey => GREY,
            Self::Green => GREEN,
            Self::Brown => BROWN,
            Self::Orange => ORANGE,
        }
    }
}
