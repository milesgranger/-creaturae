extern crate cotn;
extern crate num;

use cotn::prelude::*;
use num::pow::Pow;


#[test]
fn test_init_world() {

    let eval_y = vec![0., 0.25, 0.5, 0.75, 1.];
    let eval_x = vec![
        vec![1., 2., 1., 1., 1., 2., 1., 1.],
        vec![3., 3., 2., 2., 1., 2., 1., 2.],
        vec![7., 2., 3., 3., 1., 2., 1., 3.],
        vec![4., 1., 2., 4., 1., 2., 1., 4.],
        vec![2., 2., 1., 5., 1., 2., 1., 5.],
        vec![1., 2., 5., 1., 1., 2., 4., 1.],
        vec![1., 3., 2., 3., 2., 4., 1., 2.],
        vec![3., 2., 2., 3., 1., 3., 2., 3.],
        vec![2., 1., 2., 4., 2., 2., 1., 4.],
        vec![1., 2., 5., 2., 1., 1., 3., 5.],
        
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
        creature_factory
    );

    world.run();

}