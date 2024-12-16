use crate::prelude::*;
use super::MapArchitect;

/// Архитектор "Комнаты".
pub struct RoomsArchitect {}

/// Реализация типажа архитектора карты для архитектора "Комнаты".
impl MapArchitect for RoomsArchitect {
    /// Функция создания карты с помощью архитектора "Комнаты".
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
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.add_boundaries();
        mb.player_start = mb.rooms[0].center();
        mb.portal_start = mb.find_most_distant();

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}