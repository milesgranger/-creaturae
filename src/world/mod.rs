//! Contains different implementations of [`World`] behaviors.
//! 
//! 

use prelude::*;
use rayon::prelude::*;
use rand::prelude::*;


type Evalx = Vec<Vec<f32>>;
type Evaly = Vec<f32>;


/// The [`World`] in which [`Creature`]s live
pub struct World<'a, F, T>
    where 
        F: (Fn(f32, f32) -> f32) + Sync,
        T: Creature
{
    eval_x: &'a Evalx,
    eval_y: &'a Evaly,
    eval_func: F,
    n_generations: u32,
    n_population: u32,
    population: Vec<T>
}

impl<'a, F, T> World<'a, F, T>
    where 
        F: (Fn(f32, f32) -> f32) + Sync,
        T: Creature
{

    /// Create a new World
    pub fn new<C>(eval_x: &'a Evalx,
                  eval_y: &'a Evaly, 
                  eval_func: F, 
                  creature_factory: C) -> Self 
        where
            C: Fn() -> T
    {
        let n_population = 50;
        let n_generations = 100;
        World {
            eval_x,
            eval_y,
            eval_func,
            n_generations,
            n_population,
            population: (0..n_population)
                .map(|_| creature_factory())
                .collect::<Vec<T>>()
        }
    }

    pub fn run(&mut self) {

        let mut rng = thread_rng();

        print!("\n");
        let _ = (0..self.n_generations + 1)
            .map(|i| {
                
                // Evolve the population
                let _ = self.population
                    .iter_mut()
                    .map(|creature| creature.evolve())
                    .collect::<Vec<()>>();

                // Evaluate the population
                let sum_errors = self.population 
                    .iter()
                    .map(|creature| {
                        let predictions: Vec<f32> = creature.predict_proba(&self.eval_x);
                        predictions
                            .iter()
                            .zip(self.eval_y.iter())
                            .map(|(y_hat, y)| (self.eval_func)(*y_hat, *y))
                            .sum::<f32>()
                    })
                    .collect::<Vec<f32>>();

                // Output the total error in this population
                let avg: f32 = sum_errors.iter().sum::<f32>() / sum_errors.len() as f32;
                print!("\rGenration: {} -- Sum error: {}", i, avg);

                // Do population selection
                // TODO: Best implementation -> Now select best 25% and then 
                // randomly select from those mutating new ones until back to desired population
                let val = find_75th_quantile_value(sum_errors.clone());
                let best_of_population_idxs = sum_errors
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, error)| {
                        if *error < val {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>();

                let _ = sum_errors
                    .iter()
                    .enumerate()
                    .map(|(idx, error)| {
                        if *error > val {

                            // Replace this creature with a random mutation form a random selection of the best
                            let parent_idx: usize = rng.gen_range(0, best_of_population_idxs.len());
                            let parent_idx = best_of_population_idxs[parent_idx];
                            let mut creature = self.population[parent_idx].clone();
                            creature.evolve();
                            self.population[idx] = creature;
                        }
                    })
                    .collect::<Vec<()>>();
            })
            .collect::<Vec<()>>();
        println!("\n")
    }
}


fn find_75th_quantile_value<T: Copy + PartialOrd>(mut v: Vec<T>) -> T {
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let middle: usize = v.len() / 2;
    let slice = &v[middle..];
    slice[slice.len() / 2]
}
