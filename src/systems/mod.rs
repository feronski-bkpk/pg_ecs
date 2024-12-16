use crate::prelude::*;

mod player_input;
mod map_render;
mod entity_render;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod combat;
mod chasing;
mod fov;
mod movement_filter;
mod use_items;

/// Планировщик функций состояния "awaiting input".
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

/// Планировщик функций состояния "player turn".
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(use_items::use_items_system())
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

/// Планировщик функций состояния "monster turn".
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(chasing::chasing_system())
        .flush()
        //.add_system(use_items::use_items_system()) // на будущее
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement_filter::movement_filter_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}