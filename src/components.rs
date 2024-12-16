use crate::prelude::*;
use std::collections::HashSet;

/// Компонент "отрисовка" (цвет - символ).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType
}

/// Компонент "игрок" (уровень, на котором находится игрок).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32
}

/// Компонент "враг".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Компонент "намерение передвижения" (сущность, позиция(x,y)).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point
}

/// Компонент "намерение атаковать" (атакующий, жертва).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity
}

/// Компонент "здоровье" (текущее, максимальное).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

/// Компонент "урон" (очки урона).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);

/// Компонент "имя" (имя).
#[derive(Clone, PartialEq)]
pub struct Name (pub String);

/// Компонент "преследование игрока".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

/// Компонент "предмет".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

/// Компонент "портал".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Portal;

/// Компонент "поле зрения" (видимые плитки, радиус видимости, обновлять?).
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool
}

/// Реализация функций логики полей зрения.
impl FieldOfView {
    /// Со старта все плитки грязные.
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true
        }
    }

    /// Вспомогательная функция.
    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true
        }
    }
}

/// Компонент "предоставить лечение" (размер лечения).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32
}

/// Компонент "предоставить усиление" (размер усиления).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesPowerUp {
    pub amount: i32
}

/// Компонент "предоставить карту".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

/// Компонент "носимый" (для предметов в интентаре).
#[derive(Clone, PartialEq)]
pub struct Carried (pub Entity);

/// Компонент "активация предмета" (кем, что).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity
}