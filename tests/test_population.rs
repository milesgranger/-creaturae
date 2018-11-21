extern crate cotn;
extern crate num;

use cotn::prelude::*;
use num::pow::Pow;


#[test]
fn test_init_world() {

    let eval_y = vec![1., 0., 1., 0., 1., 0., 1., 0., 1., 0.];
    let eval_x = vec![
        vec![1., 2., 1., 1., 1., 2., 1., 1.],
        vec![3., 3., 2., 2., 1., 2., 1., 0.],
        vec![7., 2., 3., 3., 1., 2., 1., 1.],
        vec![4., 1., 2., 4., 1., 2., 1., 0.],
        vec![2., 2., 1., 5., 1., 2., 1., 1.],
        vec![1., 2., 5., 1., 1., 2., 4., 0.],
        vec![1., 3., 2., 3., 2., 4., 1., 1.],
        vec![3., 2., 2., 3., 1., 3., 2., 0.],
        vec![2., 1., 2., 4., 2., 2., 1., 1.],
        vec![1., 2., 5., 2., 1., 1., 3., 0.],
        
    ];

    let eval_func = |y: f32, y_hat: f32| {
        (y - y_hat).pow(2.)
    };
    let creature_factory = || {
        Simpleton::new(8, sigmoid)
    };

    let mut world = World::new(
        &eval_x,
        &eval_y,
        eval_func,
        2_u32,
        100_u32,
        creature_factory
    );

    world.run();

}