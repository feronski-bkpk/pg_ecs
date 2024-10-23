use crate::prelude::*;

// рождение игрока
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // создание новой сущности
    ecs.push(
        (
            Player, // тег
            pos,    // позиция
            Render{ // отрисовка
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}