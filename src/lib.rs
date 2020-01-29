use amethyst_core::math::Vector2;
use amethyst_core::timing::{Stopwatch, Time};
use amethyst_input::InputEvent;
use rltk::{Console as _, GameState, Rltk};
use specs::shred::RunNow as _;
use specs::shrev::EventChannel;
use specs::{World, WorldExt as _};

pub mod components;
pub mod controls;
pub mod engine;
pub mod systems;
pub mod terrain;

use self::components::MousePos;
use self::controls::Bindings;
use self::systems::{BuildSystem, MovementSystem, SelectionSystem, SpawnSystem, WaypointSystem};
use self::terrain::Map;

pub struct State {
    world: World,

    build_system: BuildSystem,
    movement_system: MovementSystem,
    select_system: SelectionSystem,
    spawn_system: SpawnSystem,
    waypoint_system: WaypointSystem,
}

impl State {
    pub fn new() -> Self {
        let mut game_state = State {
            world: World::new(),
            build_system: BuildSystem::default(),
            movement_system: MovementSystem::default(),
            select_system: SelectionSystem::default(),
            spawn_system: SpawnSystem::default(),
            waypoint_system: WaypointSystem::default(),
        };

        game_state
            .world
            .insert(EventChannel::<InputEvent<Bindings>>::new());
        game_state.world.insert(Map::grass(30, 30));
        game_state.world.insert(MousePos(None));
        game_state.world.insert(Stopwatch::default());
        game_state.world.insert(Time::default());

        game_state.setup_systes();

        game_state
    }

    fn run_systems(&mut self) {
        self.build_system.run_now(&self.world);
        self.movement_system.run_now(&self.world);
        self.select_system.run_now(&self.world);
        self.spawn_system.run_now(&self.world);
        self.waypoint_system.run_now(&self.world);

        self.world.maintain();
    }

    fn handle_input(&mut self, ctx: &mut Rltk) {
        let mouse_pos = ctx.mouse_pos();
        *self.world.write_resource::<MousePos>() =
            MousePos(Some(Vector2::new(mouse_pos.0, mouse_pos.1)));
    }

    fn handle_time(&mut self) {
        let mut stopwatch = self.world.write_resource::<Stopwatch>();
        let mut time = self.world.write_resource::<Time>();

        let elapsed = stopwatch.elapsed();
        stopwatch.restart();

        time.increment_frame_number();
        time.set_delta_time(elapsed);
    }

    fn setup_systes(&mut self) {
        self.world.write_resource::<Stopwatch>().start();

        self.build_system.setup(&mut self.world);
        self.movement_system.setup(&mut self.world);
        self.select_system.setup(&mut self.world);
        self.spawn_system.setup(&mut self.world);
        self.waypoint_system.setup(&mut self.world);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        use self::engine::rltk::{input, renderer};

        ctx.cls();

        self.handle_time();
        self.handle_input(ctx);
        input::handle_input(ctx, self.world.system_data());

        self.run_systems();

        renderer::draw_map(ctx, &*self.world.fetch::<Map>());
        renderer::draw_selections(ctx, self.world.system_data());
        renderer::draw_enemies(ctx, self.world.system_data());
        renderer::draw_buildings(ctx, self.world.system_data());
    }
}
