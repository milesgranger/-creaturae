extern crate cotn;


use cotn::prelude::*;


#[test]
fn test_init_creature() {
    let creature = Creature::new()
        .set_brain(LogisticRegressionBrain::new());
}