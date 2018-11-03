//! Contains different implementations of [`World`] behaviors.
//! 
//! 

use prelude::*;


type Evalx = Vec<Vec<f32>>;
type Evaly = Vec<f32>;


/// The [`World`] in which [`Creature`]s live
pub struct World<'a, F, B> 
    where 
        F: (Fn(f32, f32) -> f32) + Sync,
        B: Brain
{
    eval_x: &'a Evalx,
    eval_y: &'a Evaly,
    eval_func: F,
    n_population: u32,
    population: Vec<Creature<B>>
}

impl<'a, F, B> World<'a, F, B>
    where 
        F: (Fn(f32, f32) -> f32) + Sync,
        B: Brain
{

    /// Create a new World
    pub fn new<C>(eval_x: &'a Evalx, 
                  eval_y: &'a Evaly, 
                  eval_func: F, 
                  creature_factory: C) -> Self 
        where C: Fn() -> Creature<B>
    {
        let n_population = 100;
        World {
            eval_x,
            eval_y,
            eval_func,
            n_population,
            population: (0..n_population)
                .map(|_| creature_factory())
                .collect::<Vec<Creature<B>>>()
        }
    }
}
