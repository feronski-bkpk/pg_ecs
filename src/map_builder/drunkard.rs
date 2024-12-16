use crate::prelude::*;
use super::MapArchitect;

/// Архитектор "Пьяная прогулка".
pub struct DrunkardsWalkArchitect {}
const STEPS_NUM: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

/// Реализация типажа архитектора карты для архитектора "Пьяная прогулка".
impl MapArchitect for DrunkardsWalkArchitect {
    /// Функция создания карты с помощью архитектора "Пьяная прогулка".
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            portal_start: Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH /2, SCREEN_HEIGHT /2);
        self.drunkard(&center, rng, &mut mb.map);

        while mb.map.tiles.iter().filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR {
            self.drunkard(
                &Point::new(
                    rng.range(0, SCREEN_WIDTH),
                    rng.range(0, SCREEN_HEIGHT)
                ),
                rng,
                &mut mb.map
            );
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.
            );
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall)
        }
        mb.add_boundaries();

        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.portal_start = mb.find_most_distant();
        mb
    }
}

/// Реализация функций логики архитектора "Пьяная прогулка".
impl DrunkardsWalkArchitect {
    /// Функция логики "пьяницы". Плитки, на которые наступает пьяница, становятся плитками пола.
    /// Передвижение "пьяницы" ограничивается числом шагов.
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut steps_num = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1
            }
            if !map.in_bounds(drunkard_pos) { break }

            steps_num +=1;
            if steps_num > STEPS_NUM { break }
        }
    }
}