use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());

    if let Some(key) = *key {
        let mut regeneration = false;
        let mut delta= Point::zero();

        match key {
            VirtualKeyCode::Escape => {
                return *turn_state = TurnState::Exit
            },
            VirtualKeyCode::Space => {
                regeneration = true;
            },
            VirtualKeyCode::Left | VirtualKeyCode::A | VirtualKeyCode::Numpad4 => {
                delta = Point::new(-1, 0)
            },
            VirtualKeyCode::Right | VirtualKeyCode::D | VirtualKeyCode::Numpad6 => {
                delta = Point::new(1, 0)
            },
            VirtualKeyCode::Up | VirtualKeyCode::W | VirtualKeyCode::Numpad8 => {
                delta = Point::new(0, -1)
            },
            VirtualKeyCode::Down | VirtualKeyCode::S | VirtualKeyCode::Numpad2 => {
                delta = Point::new(0, 1)
            },
            _ => {
                return;
            }
        }

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
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

        if regeneration {
            if let Ok(health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + 1)
            }
        }

        *turn_state = TurnState::PlayerTurn
    }
}