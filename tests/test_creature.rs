extern crate creaturae;


use creaturae::prelude::*;


#[test]
fn test_init_creature() {
    let creature = Creature::new()
        .set_brain(LogisticRegressionBrain::new());
}