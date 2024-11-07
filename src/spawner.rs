use crate::prelude::*;

// рождение игрока
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // создание новой сущности
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Health {
                current: 20,
                max: 20
            }
        )
    );
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {

    let (hp, name, glyph) = match rng.roll_dice(1,10) {
        1..=8 => goblin(),
        _ => orc()
    };

    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph
            },
            MovingRandomly{},
            Health{ current: hp, max: hp },
            Name(name)
        )
    );
}