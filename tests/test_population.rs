extern crate cotn;

use cotn::prelude::*;


#[test]
fn test_init_world() {

    let eval_y = vec![1., 2., 3., 4., 5.];
    let eval_x = vec![
        vec![1., 2., 3., 1.],
        vec![1., 2., 3., 2.],
        vec![1., 2., 3., 3.],
        vec![1., 2., 3., 4.],
        vec![1., 2., 3., 5.],
    ];

    let eval_func = |y: f32, y_hat: f32| {
        (y - y_hat).abs()
    };
    let creature_factory = || {
        Creature::new(Simpleton::new(4, sigmoid))
    };

    let world = World::new(
        &eval_x,
        &eval_y,
        eval_func,
        creature_factory
    );
}