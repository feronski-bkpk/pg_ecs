use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
// тег для сущности
pub struct Player;