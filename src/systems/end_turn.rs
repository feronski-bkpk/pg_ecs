use crate::prelude::*;

/// Система "конец хода". Она обрабатывает смену игрового состояния.
#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Portal)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] map: &Map
) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut portal = <&Point>::query().filter(component::<Portal>());

    let portal_default = Point::new(-1, -1);
    let portal_pos = portal
        .iter(ecs)
        .nth(0)
        .unwrap_or(&portal_default);

    let current_state = turn_state.clone();

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current <1 {
            new_state = TurnState::GameOver
        }
        if pos == portal_pos {
            new_state = TurnState::Victory
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel
        }
    });

    *turn_state = new_state
}