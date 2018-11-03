extern crate cotn;


use cotn::prelude::*;


#[test]
fn test_init_creature() {
    let mut creature = Creature::new(Simpleton::new(10, sigmoid));
    creature.evolve();
}

#[test]
fn test_predict_proba() {
    let creature = Creature::new(Simpleton::new(5, sigmoid));

    let x = vec![
        vec![1_f32, 2., -3., -40., 5.],
        vec![2_f32, -3., -4., 5., 6.]
    ];

    let predictions = creature.predict_proba(&x);
    println!("Predictions: {:#?}", predictions);
    let any_above_1 = predictions
        .iter()
        .any(|v| *v > 1.0);
    assert!(!any_above_1);
}