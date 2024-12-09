#![windows_subsystem="windows"]
#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
//    pub const TEXTURE_ASCII_X32: &str = "terminal32x32.png";
    pub const TEXTURE_ASCII_X8: &str = "terminal8x8.png";
    pub const TEXTURE_DUNGEON: &str = "dungeon_texture.png";
    pub const FINAL_LEVEL: u32 = 2;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH/2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT/2;
    pub const SCREEN_WIDTH: i32 = 70;
    pub const SCREEN_HEIGHT: i32 = 40;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}
use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
}
impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        //spawn_portal(&mut ecs, map_builder.portal_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.portal_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        map_builder.monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut ecs, &mut rng, *pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::GameStart);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);
        //spawn_portal(&mut self.ecs, map_builder.portal_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.portal_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        map_builder.monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e)
            }
        }
        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            }
        );

        if map_level == FINAL_LEVEL {
            spawn_portal(&mut self.ecs, map_builder.portal_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.portal_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit
        }

        map_builder.monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn game_start_screen(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 16, YELLOW, BLACK, "=== Introduction ===");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 14, WHITE, BLACK, "Ferris was the leader of a terrifying band of mercenaries. They were");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 12, WHITE, BLACK, "in the service of the Kingdom, but one day the King, overcome with");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 10, WHITE, BLACK, "fear, ordered all the members of the squad to be executed, and");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 8, WHITE, BLACK, "the leader was sent to dangerous lands teeming with monsters.");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 6, WHITE, BLACK, "You need to find a way out and try not to die.");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 2, YELLOW, BLACK, "=== Controls ===");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 0, WHITE, BLACK, "Up arrow || W || Numpad8 -> Move Up");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 2, WHITE, BLACK, "Left arrow || A || Numpad4 -> Move Left");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 4, WHITE, BLACK, "Down arrow || S || Numpad2 -> Move Down");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 6, WHITE, BLACK, "Right arrow || D || Numpad6 -> Move Right");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 8, WHITE, BLACK, "Keys 1-9 -> Activate items");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 10, WHITE, BLACK, "SPACE -> Pick up item");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 12, WHITE, BLACK, "Escape -> Exit");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 16, GREEN, BLACK, "Press ENTER to start.");

        if ctx.key.is_some() {
            match ctx.key.unwrap() {

                VirtualKeyCode::Escape => {
                    self.resources.insert(TurnState::Exit)
                }

                VirtualKeyCode::Return => {
                    self.resources.insert(TurnState::AwaitingInput)
                }

                _ => {}
            }
        }
    }

    fn game_over_screen(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(SCREEN_HEIGHT/2 -6, RED, BLACK, "YOU DIED");
        ctx.print_color_centered(SCREEN_HEIGHT/2 -4, WHITE, BLACK, "You were torn apart by the terrible monsters of the dangerous lands.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 -2, WHITE, BLACK, "What is left of you will continue to be food for the");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +0, WHITE, BLACK, "inhabitants of these places for a long time.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +2, YELLOW, BLACK, "You can try again if you are brave enough.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +6, GREEN, BLACK, "Press ENTER to start a new game.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +8, GREEN, BLACK, "Press Escape to exit.");

        if ctx.key.is_some() {
            match ctx.key.unwrap() {

                VirtualKeyCode::Escape => {
                    self.resources.insert(TurnState::Exit)
                }

                VirtualKeyCode::Return => {
                    self.reset_game_state()
                }

                _ => {}
            }
        }
    }

    fn victory_screen(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(SCREEN_HEIGHT/2 -6, GREEN, BLACK, "YOU WIN");
        ctx.print_color_centered(SCREEN_HEIGHT/2 -4, WHITE, BLACK, "The mysterious portal has led you out of dangerous lands.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 -2, WHITE, BLACK, "But who knows if your journey has come to an end...");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +0, YELLOW, BLACK,"You can try again and have more fun.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +4, GREEN, BLACK, "Press ENTER to start a new game.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +6, GREEN, BLACK, "Press Escape to exit.");

        if ctx.key.is_some() {
            match ctx.key.unwrap() {

                VirtualKeyCode::Escape => {
                    self.resources.insert(TurnState::Exit)
                }

                VirtualKeyCode::Return => {
                    self.reset_game_state()
                }

                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(
                &mut self.ecs,
                &mut self.resources
            ),
            TurnState::PlayerTurn => self.player_systems.execute(
                &mut self.ecs,
                &mut self.resources
            ),
            TurnState::MonsterTurn => self.monster_systems.execute(
                &mut self.ecs,
                &mut self.resources
            ),
            TurnState::GameStart => {
                self.game_start_screen(ctx)
            }
            TurnState::GameOver => {
                self.game_over_screen(ctx)
            }
            TurnState::NextLevel => {
                self.advance_level()
            }
            TurnState::Victory => {
                self.victory_screen(ctx)
            }
            TurnState::Exit => {
                ctx.quitting = true
            }
        }
        render_draw_buffer(ctx).expect("Render error")
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("PG-game")
        .with_fps_cap(30.)
        .with_fullscreen(true)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(TEXTURE_DUNGEON, 32, 32)
        .with_font(TEXTURE_ASCII_X8, 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, TEXTURE_DUNGEON)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, TEXTURE_DUNGEON)
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, TEXTURE_ASCII_X8)
        .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, TEXTURE_ASCII_X8)
        .build()?;

    main_loop(context, State::new())
}

// 0 - TEXTURE_DUNGEON   -- background
// 1 - TEXTURE_DUNGEON   -- entities
// 2 - ASCII_x8 (big)    -- game screen (win or lose)
// 3 - ASCII_x8 (small)  -- hud and tooltips

// для wall_hack:
// в map_render.rs комменть " && (player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]) "
// в map_render.rs меня на true " let tint = if player_fov.visible_tiles.contains(&pt) "
// в entity_render комменть " .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos)) "