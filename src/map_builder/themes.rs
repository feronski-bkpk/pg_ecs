use crate::prelude::*;

/// Дизайнер "Подземелья".
pub struct DungeonTheme {}

/// Реализация функций логики дизайнера "Подземелья".
impl DungeonTheme {
    /// Функция сборки темы дизайнера "Подземелья".
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

/// Реализация функций логики дизайнера "Подземелья".
impl MapTheme for DungeonTheme {
    /// Функция сопоставления плиток и символов "подземелья".
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>')
        }
    }
}

/// Дизайнер "Леса".
pub struct ForestTheme {}

/// Реализация функций логики дизайнера "Леса".
impl ForestTheme {
    /// Функция сборки темы дизайнера "Леса".
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

/// Реализация функций логики дизайнера "Леса".
impl MapTheme for ForestTheme {
    /// Функция сопоставления плиток и символов "подземелья".
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>')
        }
    }
}