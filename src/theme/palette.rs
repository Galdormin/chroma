use bevy::color::Color;

/* === GAME COLORS === */

pub const BACKGROUND: Color = Color::srgb_u8(73, 52, 61);

pub const WHITE: Color = Color::srgb_u8(210, 201, 165);
pub const GREY: Color = Color::srgb_u8(86, 84, 110);
pub const GREEN: Color = Color::srgb_u8(102, 132, 95);
pub const BROWN: Color = Color::srgb_u8(138, 88, 101);
pub const RED: Color = Color::srgb_u8(182, 92, 95);
pub const YELLOW: Color = Color::srgb_u8(188, 133, 99);

/// All biome colors
pub const BIOME_COLORS: &[Color] = &[GREY, GREEN, BROWN, RED, YELLOW];

/* === UI COLORS === */

pub const LABEL_TEXT: Color = YELLOW;
pub const HEADER_TEXT: Color = RED;
pub const BUTTON_TEXT: Color = WHITE;
