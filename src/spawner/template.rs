use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

/// Состав сущностей в файле template.
#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub fov: Option<i32>,
    pub base_damage: Option<i32>
}

/// Перечисление типов сущностей.
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item
}

/// Состав файла template.
#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>
}

/// Реализация функций логики templates.
impl Templates {
    /// Функция загрузки файла template.
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    /// Функция создания сущностей вообще.
    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point]
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            }
        );

        let mut commands = CommandBuffer::new(ecs);
        spawn_points
            .iter()
            .for_each(|pt| {
                if let Some(entity) = rng.random_slice_entry(&available_entities) {
                    self.spawn_entity(pt, entity, &mut commands);
                }
            }
        );
        commands.flush(ecs)
    }

    /// Функция создания конкретной сущности с определёнными параметрами.
    fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut CommandBuffer
    ) {
        let entity = commands.push(
            (
                pt.clone(),
                Render {
                    color: ColorPair::new(WHITE, BLACK),
                    glyph: to_cp437(template.glyph)
                },
                Name(template.name.clone())
            )
        );

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item{}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy{});
                commands.add_component(entity, FieldOfView::new(template.fov.unwrap()));
                commands.add_component(entity, ChasingPlayer{});
                commands.add_component(entity, Health{
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap()
                });
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| {
                    match provides.as_str() {
                        "Healing" => commands.add_component(entity, ProvidesHealing{ amount: *n }),
                        "StrangeScroll" => commands.add_component(entity, ProvidesDungeonMap{}),
                        "PowerUp" => commands.add_component(entity, ProvidesPowerUp{ amount: *n }),
                        _ => println!("Warning: unknown provide {}", provides)
                    }
                }
            )
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage))
        }
    }
}