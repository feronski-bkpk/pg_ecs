use crate::prelude::*;
use super::MapArchitect;

/// Архитектор "Клеточный автомат".
pub struct CellularAutomataArchitect {}

/// Реализация типажа архитектора карты для архитектора "Клеточный автомат".
impl MapArchitect for CellularAutomataArchitect {
    /// Функция создания карты с помощью архитектора "Клеточный автомат".
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            portal_start: Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };

        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        mb.add_boundaries();

        let start = self.find_start(&mb.map);

        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.portal_start = mb.find_most_distant();
        mb
    }
}

/// Реализация функций логики архитектора "Клеточный Автомат".
impl CellularAutomataArchitect {
    /// Функция создания карты шума.
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);

            if roll > 55 {
                *t = TileType::Floor
            } else {
                *t = TileType::Wall
            }
        })
    }

    /// Функция подсчёта числа соседей.
    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbours = 0;

        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix==0 && iy==0) && map.tiles[map_idx(x+ix, y+iy)] == TileType::Wall {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    /// Функция прохождения по всем плиткам. Проверяет условия жизни и смерти клеток на карте.
    /// Если у клетки соседей больше 4 или равно 0, то клетка умирает, иначе живёт дальше.
    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1..SCREEN_HEIGHT -1 {
            for x in 1..SCREEN_WIDTH -1 {
                let neighbors = self.count_neighbours(x, y, map);
                let idx = map_idx(x, y);

                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall
                } else {
                    new_tiles[idx] = TileType::Floor
                }
            }
        }
        map.tiles = new_tiles;
    }

    /// Функция нахождения стартовой позиции.
    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);

        let closest_point = map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx))))
            .min_by(|(_, distance1), (_, distance2)| distance1.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}