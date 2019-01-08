use amethyst;
use amethyst::core::nalgebra::Vector2;
use amethyst::ecs::prelude::*;
use amethyst_test::AmethystApplication;
use simple_towers::enemy::Enemy;
use simple_towers::movement::{Movement, Waypoint, WaypointSystem};

#[test]
fn waypoint_sets_movement() -> Result<(), amethyst::Error> {
    AmethystApplication::blank()
        .with_system(WaypointSystem, "", &[])
        .with_setup(|world| {
            world
                .create_entity()
                .with(Movement {
                    pos: Vector2::new(10.0, 10.0),
                    vel: Vector2::new(0.0, 0.0),
                })
                .with(Enemy { speed: 1.0 })
                .with(Waypoint::new(&[
                    Vector2::new(10.0, 0.0),
                    Vector2::new(0.0, 10.0),
                ]))
                .build();
        })
        .with_assertion(|world| {
            let moves = world.system_data::<ReadStorage<Movement>>();
            let mv = moves.join().next().unwrap();

            // position hasn't changed
            assert_eq!(mv.pos, Vector2::new(10.0, 10.0));

            // velocity points towards the first goal
            assert_eq!(mv.vel, Vector2::new(0.0, -1.0));
        })
        .run()
}
