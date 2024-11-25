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
    pub const TEXTURE_ASCII_X32: &str = "terminal32x32.png";
    pub const TEXTURE_ASCII_X8: &str = "terminal8x8.png";
    pub const TEXTURE_DUNGEON: &str = "dungeon_texture.png";
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
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        spawn_portal(&mut ecs, map_builder.portal_start);

        map_builder.monster_spawns
            .iter()
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, *pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::GameStart);

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
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_portal(&mut self.ecs, map_builder.portal_start);

        map_builder.monster_spawns
            .iter()
            .for_each(|pos| spawn_monster(&mut self.ecs, &mut rng, *pos));

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput)
    }

    fn game_start_screen(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 16, YELLOW, BLACK, "=== Introduction ===");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 14, WHITE, BLACK, "Ferris was the leader of a terrifying band of mercenaries. They were");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 12, WHITE, BLACK, "in the service of the Kingdom, but one day the King, overcome with");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 10, WHITE, BLACK, "fear, ordered all the members of the squad to be executed, and the");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 8, WHITE, BLACK, "leader to be sent to the cave to the monsters. You need to find a");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 6, WHITE, BLACK, "way out and try not to die.");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 - 2, YELLOW, BLACK, "=== Controls ===");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 0, WHITE, BLACK, "Up arrow || W || Numpad8 -> Move Up");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 2, WHITE, BLACK, "Left arrow || A || Numpad4 -> Move Left");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 4, WHITE, BLACK, "Down arrow || S || Numpad2 -> Move Down");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 6, WHITE, BLACK, "Right arrow || D || Numpad6 -> Move Right");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 8, WHITE, BLACK, "SPACE -> Health Regeneration");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 10, WHITE, BLACK, "Escape -> Exit");
        ctx.print_color_centered(SCREEN_HEIGHT / 2 + 14, GREEN, BLACK, "Press ENTER to start.");

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
        ctx.print_color_centered(SCREEN_HEIGHT/2 -4, WHITE, BLACK, "You have been torn apart by the terrible monsters of the dungeon.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 -2, WHITE, BLACK, "Your remains will rot in the depths of the caves for a long time.");
        ctx.print_color_centered(SCREEN_HEIGHT/2 +0, YELLOW, BLACK, "You can try again if you are brave enough.");
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

    fn victory_screen(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(SCREEN_HEIGHT/2 -6, GREEN, BLACK, "YOU WIN");
        ctx.print_color_centered(SCREEN_HEIGHT/2 -4, WHITE, BLACK, "A bright light blinded you... ");
        ctx.print_color_centered(SCREEN_HEIGHT/2 -2, WHITE, BLACK, "The portal sent you straight to the surface!");
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