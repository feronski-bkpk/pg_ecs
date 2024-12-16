use crate::prelude::*;

/// Система "бой". Обрабатывает все сущности с сообщениями о намерении "атаки". Учитывает урон атакующей сущности.
/// Если у сущности не осталось здоровья, то её удаляют из игры.
#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim) )
        .collect();

    victims
        .iter()
        .for_each(|(message, attacker, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let damage = if let Ok(attacker) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = attacker.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= damage;
            if health.current <1 && !is_player { commands.remove(*victim) }
        }
        commands.remove(*message)
    })
}