use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::A | VirtualKeyCode::Numpad4 => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D | VirtualKeyCode::Numpad6 => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::W | VirtualKeyCode::Numpad8 => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::S | VirtualKeyCode::Numpad2 => Point::new(0, 1),
            _ => Point::zero()
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos+delta)) )
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;

                    commands
                        .push(((), WantsToAttack {
                            attacker: player_entity,
                            victim: *entity
                        }));
                });

            if !hit_something {
                commands.push(((), WantsToMove { entity: player_entity, destination }));
            }
        }
        *turn_state = TurnState::PlayerTurn
    }
}