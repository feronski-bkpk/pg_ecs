use crate::prelude::*;

/// Система "использование предмета". Обрабатывает все сущности с сообщениями о намерении "использования предмета".
#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[read_component(ProvidesPowerUp)]
#[write_component(Damage)]
pub fn use_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    let mut power_up_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);

            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount))
                }

                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true)
                }

                if let Ok(power_up) = item.get_component::<ProvidesPowerUp>() {
                    power_up_to_apply.push((activate.used_by, power_up.amount))
                }
            }
            commands.remove(activate.item);
            commands.remove(*entity);
        });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(
                    health.max,
                    health.current + heal.1
                );
            }
        }
    }

    for power_up in power_up_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(power_up.0) {
            if let Ok(damage) = target.get_component_mut::<Damage>() {
                damage.0 += power_up.1;
            }
        }
    }
}