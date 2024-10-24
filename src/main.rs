#![windows_subsystem="windows"]
#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

// любой модуль, импортирующий прелюдию, будет иметь доступ ко всем её частям //
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH/2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT/2;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}
use prelude::*;

// чертёж игры
struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule
}
impl State {
    // создание новой игры
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, pos, &mut rng));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}
// для хранения состояния игры (между кадрами)
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error")
    }
}

// main возвращает тип Result (из bracket-lib)
fn main() -> BError {
    // создание терминала
    let context = BTermBuilder::new()
        .with_title("PG-game")
        .with_fps_cap(30.)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    // запуск игрового цикла (новой игры)
    main_loop(context, State::new())
}