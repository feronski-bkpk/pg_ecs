use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub portal_start: Point
}
impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            portal_start: Point::zero()
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center(); // положение персонажа в центр первой комнаты

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(mb.player_start)],
            &mb.map,
            1024.
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        mb.portal_start = mb.map.index_to_point2d(
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_,dist)| *dist < UNREACHABLE)
            .max_by(|a,b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0
        );
        mb
    }

    // заполнение карты
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    // строитель комнат ("вырезает комнаты из плиток-стен")
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // продолжает генерировать комнаты до тех пор, пока не станет NUM_ROOMS комнат
        while self.rooms.len() < NUM_ROOMS {
            // генерация случайной комнаты (случайные координаты и размер - x,y,w,h)
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),   // x (ограничение по ширине экрана)
                rng.range(1, SCREEN_HEIGHT - 10),  // y (ограничение по высоте экрана)
                rng.range(2, 10),                  // h (ограничение по высоте прямоугольника)
                rng.range(2, 10)                   // w (ограничение по ширине прямоугольника)
            );
            // флажок наложения (пересечения) комнат
            let mut overlap = false;
            // проверка новой комнаты на наложение (пересечение) со старыми
            for r in self.rooms.iter() {
                // intersect - функция bracket-lib
                if r.intersect(&room) {
                    overlap = true
                }
            }
            // если комната не накладывается (пересекается)
            if !overlap {
                // убеждаемся, что плитки комнаты находятся в пределах карты
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        // получение индекса вектора для соответствующей плитки на карте
                        let idx = map_idx(p.x, p.y);
                        // изменяю состояние плитки карты находя плитку по индексу вектора и
                        // назначаю ей тип -> пол
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                // добавляем комнату в вектор
                self.rooms.push(room)
            }
        }
    }

    // функция строительства вертикальных коридоров
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1,y2) ..= max(y1,y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x,y)) {
                self.map.tiles[idx as usize] = TileType::Floor
            }
        }
    }

    // функция строительства горизонтальных коридоров
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1,x2) ..= max(x1,x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x,y)) {
                self.map.tiles[idx as usize] = TileType::Floor
            }
        }
    }

    // функция строительства коридоров между комнатами
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // сортировка комнат по центральным x (для более лаконичных коридоров)
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            // хаотичное (случайное) копание коридоров
            if rng.range(0,2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y)
            }
        }
    }
}