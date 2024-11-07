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

        players.iter(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;
            commands.push(((), WantsToMove{ entity: *entity, destination }));
        });
        *turn_state = TurnState::PlayerTurn
    }
}