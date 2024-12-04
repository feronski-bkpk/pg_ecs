use crate::prelude::*;

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push(
        (
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!')
        },
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 }
        )
    );
}

pub fn spawn_strange_scroll(ecs: &mut World, pos: Point) {
    ecs.push(
        (
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{')
        },
        Name("Strange Scroll".to_string()),
        ProvidesDungeonMap
        )
    );
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1,6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_strange_scroll(ecs, pos),
        _ => spawn_monster(ecs, rng, pos)
    }
}

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
        Name("Strange Portal".to_string())
        )
    );
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Name("Ferris".to_string()),
            Health { current: 10, max: 10 },
            FieldOfView::new(8)
        )
    );
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblins".to_string(), to_cp437('g'))
}

fn reptile() -> (i32, String, FontCharType) {
    (3, "Croco".to_string(), to_cp437('c'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator,  pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1,100) {
        1..40 => goblin(),
        40..=80 => reptile(),
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
            ChasingPlayer,
            Name(name),
            Health{ current: hp, max: hp },
            FieldOfView::new(6)
        )
    );
}