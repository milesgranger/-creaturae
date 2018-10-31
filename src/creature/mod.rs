//! Contains the [`Creature`] implementation
//! 

use prelude::*;


pub struct Creature<B>
    where B: Brain
{
    brain: Option<B>
}


impl<B> Creature<B>
    where B: Brain
{
    pub fn new() -> Self {
        Creature {
            brain: None
        }   
    }

    pub fn set_brain(mut self, brain: B) -> Self {
        self.brain = Some(brain);
        self
    }
}
