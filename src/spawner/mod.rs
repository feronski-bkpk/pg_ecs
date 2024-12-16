mod template;

use template::Templates;
use crate::prelude::*;

/// Функция создания портала.
pub fn spawn_portal(ecs: &mut World, pos: Point) {
    ecs.push(
        (
        Item,
        Portal,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|')
        },
        Name("Mysterious Portal".to_string())
        )
    );
}

/// Функция создания игрока.
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player { map_level: 0 },
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Name("Ferris".to_string()),
            Health { current: 10, max: 10 },
            Damage(1),
            FieldOfView::new(8)
        )
    );
}

/// Функция создания уровня.
pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point]
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points)
}