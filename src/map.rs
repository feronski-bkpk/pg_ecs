use crate::prelude::*;

// число всех плиток на карте
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// безопасное назначение, копирование и сравнение
#[derive(Copy, Clone, PartialEq)]
// возможные типы плиток
pub enum TileType {
    Wall,
    Floor
}

// индексирование карты ("row-first encoding") ("сначала строка")
// y - целая часть при делении -> idx/width
// x - остаток при делении -> idx%width
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

// чертёж карты
pub struct Map {
    pub tiles: Vec<TileType>
}
impl Map {
    // создание новой карты
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // находится ли точка(x,y) в пределах карты?
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // можно ли ходить на плитку?
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // "определение нерушимости границ карты и её пределов"
    // если запрашиваемые координаты находятся в пределах карты (не включая границу), то
    // вернуть индекс запрашиваемой плитки (как следствие, разрешить действие с координатой)
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        // если запрашиваемые координаты выходят за пределы карты - вернуть None
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

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

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
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

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras // Пифагорово расстояние между двумя точками
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2)
            )
    }
}