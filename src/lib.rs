use amethyst_core::math::Vector2;
use amethyst_core::timing::{Stopwatch, Time};
use amethyst_input::InputEvent;
use rltk::{Console as _, DrawBatch, GameState, Rltk};
use specs::shred::{Dispatcher, DispatcherBuilder};
use specs::shrev::EventChannel;
use specs::{World, WorldExt as _};

pub mod components;
pub mod controls;
pub mod engine;
pub mod map;
pub mod systems;
pub mod utils;

use self::components::SelectionType;
use self::controls::{Bindings, MousePos};
use self::map::Map;
use self::systems::{BuildSystem, BuildingPlacementSystem, MovementSystem, SpawnSystem, WaypointSystem};

pub struct State {
    world: World,

    // lifetimes are those of regular and thread local systems respectively
    dispatcher: Dispatcher<'static, 'static>,
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(EventChannel::<InputEvent<Bindings>>::new());
        world.insert(Map::grass(40, 40));
        world.insert(MousePos(None));
        world.insert(SelectionType::Hover);
        world.insert(Stopwatch::default());
        world.insert(Time::default());

        let dispatcher = DispatcherBuilder::new()
            .with(BuildSystem::default(), "", &[])
            .with(BuildingPlacementSystem::default(), "", &[])
            .with(MovementSystem::default(), "", &[])
            .with(SpawnSystem::default(), "", &[])
            .with(WaypointSystem::default(), "", &[])
            .build();

        State { world, dispatcher }
    }

    fn run_systems(&mut self) {
        self.dispatcher.dispatch(&self.world);
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

    pub fn setup_systems(&mut self) {
        self.world.write_resource::<Stopwatch>().start();
        self.dispatcher.setup(&mut self.world);
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

        let mut batch = DrawBatch::new();
        batch.target(0);
        batch.cls();
        batch.target(1);
        batch.cls();

        renderer::draw_map(&mut batch, self.world.system_data());
        renderer::draw_selections(&mut batch, self.world.system_data());
        renderer::draw_enemies(&mut batch, self.world.system_data());
        renderer::draw_buildings(&mut batch, self.world.system_data());
        renderer::draw_ui(&mut batch, self.world.system_data());

        batch.submit(0);

        rltk::render_draw_buffer(ctx);
    }
}
