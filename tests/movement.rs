use amethyst;
use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::*;
use amethyst_test::AmethystApplication;
use simple_towers::components::{MoveOrder, Pos, Vel};
use simple_towers::systems::MovementSystem;

#[test]
fn path_sets_move_order_and_vel() -> Result<(), amethyst::Error> {
    AmethystApplication::blank()
        .with_system(MovementSystem, "", &[])
        .with_setup(|world| {
            world
                .create_entity()
                .with(Pos(Vector2::new(10.0, 10.0)))
                .with(Vel(Vector2::new(0.0, 0.0)))
                .with(MoveOrder::new(Vector2::new(10, 0)))
                .build();
        })
        .with_assertion(|world| {
            let (pos, vel, move_order) = world.system_data::<(
                ReadStorage<Pos>,
                ReadStorage<Vel>,
                ReadStorage<MoveOrder>
            )>();
            let (pos, vel, move_order) = (&pos, &vel, (&move_order).maybe())
                .join()
                .next()
                .unwrap();

            // position hasn't changed
            assert_eq!(pos.0, Vector2::new(10.0, 10.0));

            // velocity points towards the goal
            assert_eq!(vel.0, Vector2::new(0.0, -1.0));

            assert!(move_order.is_some());
            assert_eq!(move_order.unwrap().goal(), Some(&Vector2::new(10, 0)));
        })
        .run()
}
