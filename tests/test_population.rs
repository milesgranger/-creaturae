extern crate cotn;

use cotn::prelude::*;


#[test]
fn test_init_world() {

    let eval_y = vec![0., 0.25, 0.5, 0.75, 1.];
    let eval_x = vec![
        vec![1., 2., 1., 1., 1., 2., 1., 1.],
        vec![1., 3., 2., 2., 1., 2., 1., 2.],
        vec![1., 2., 3., 3., 1., 2., 1., 3.],
        vec![1., 1., 2., 4., 1., 2., 1., 4.],
        vec![1., 2., 1., 5., 1., 2., 1., 5.],
    ];

    let eval_func = |y: f32, y_hat: f32| {
        (y - y_hat).abs()
    };
    let creature_factory = || {
        Creature::new(Simpleton::new(8, sigmoid))
    };

    let mut world = World::new(
        &eval_x,
        &eval_y,
        eval_func,
        creature_factory
    );

    world.run();

}