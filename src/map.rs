use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/// Перечисление типов плиток.
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit
}

/// Индексирование карты ("row-first encoding" - "сначала строка").
/// Получение координат: y = idx / width, x = idx % width.
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

/// Структура карты.
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>
}

/// Реализация функций логики карты.
impl Map {
    /// Конструктор карты.
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES]
        }
    }

    /// Определяет доступность плитки (её наличие в границах карты).
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Определяет возможность передвижения.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && (
            self.tiles[map_idx(point.x, point.y)] == TileType::Floor ||
            self.tiles[map_idx(point.x, point.y)] == TileType::Exit
        )
    }

    /// Определение инедкса плитки.
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    /// Функция определения действительности перехода.
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Реализация типажа "Algorithm2D".
impl Algorithm2D for Map {
    /// Функция определения границ карты.
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    /// Функция определения доступности плитки.
    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

/// Реализация типажа "BaseMap".
impl BaseMap for Map {
    /// Функция создания вектора действительных выходов.
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.))
        }
        exits
    }

    /// Функция возвращения расстояния пути.
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras // Пифагорово расстояние между двумя точками
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2)
            )
    }

    /// Функция определения непрозрачности плитки.
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}