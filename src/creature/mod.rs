//! Contains the [`Creature`] implementation
//! 

use prelude::*;


/// The [`Creature`]
pub struct Creature<B>
    where B: Brain
{
    brain: B
}


impl<B> Creature<B>
    where B: Brain
{
    /// Construct a new creature
    pub fn new(brain: B) -> Self {
        Creature {
            brain
        }   
    }

    /// Set the [`Brain`] of this creature.
    pub fn set_brain(mut self, brain: B) -> Self {
        self.brain = brain;
        self
    }

    /// Evolve the features of the creature
    pub fn evolve(&mut self) {

        // Evolve the brain
        self.brain.evolve();
    }

    /// Predict an outcome
    pub fn predict_proba(&self, x: &Vec<Vec<f32>>) -> Vec<f32> {
        self.brain.predict_proba(x)
    } 
}
