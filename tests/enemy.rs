use amethyst;
use amethyst::core::nalgebra::Vector2;
use amethyst::ecs::prelude::*;
use amethyst_test::AmethystApplication;
use simple_towers::enemy::{Enemy, EnemySystem, MovementOrder};
use simple_towers::movement::{MovementSystem, Position, Velocity};

#[test]
fn orients_towards_movement_goal() -> Result<(), amethyst::Error> {
    AmethystApplication::blank()
        .with_system(MovementSystem, "movement", &[])
        .with_system(EnemySystem, "enemy", &["movement"])
        .with_setup(|world| {
            world
                .create_entity()
                .with(Position(Vector2::new(0.0, 0.0)))
                .with(Velocity(Vector2::new(0.0, 0.0)))
                .with(Enemy { speed: 1.0 })
                .with(MovementOrder {
                    goal: Vector2::new(10.0, 0.0),
                })
                .build();
        })
        .with_assertion(|world| {
            let vels = world.system_data::<ReadStorage<Velocity>>();
            assert_eq!(
                vels.join().next().unwrap(),
                &Velocity(Vector2::new(1.0, 0.0)),
            );
        })
        .run()
}
