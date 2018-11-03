//! Contains different [`Brain`] implementations for creatures to use.
//! 
//! 
//! 

use rayon::prelude::*;
use num::pow::Pow;
use rand::{self, Rng, thread_rng, distributions::Uniform};

pub fn sigmoid(x: f32) -> f32 {
    1. / (1_f32.pow(-x))
}


// Brain behavior
pub trait Brain {
    fn evolve(&mut self);
    fn predict_proba(&self, &Vec<Vec<f32>>) -> Vec<f32>;
}

#[derive(Clone)]
pub struct Simpleton<F>
    where F: Fn(f32) -> f32 + Sync
{
    state: Vec<f32>,
    func: F
}

impl<F> Simpleton<F> 
    where F: Fn(f32) -> f32 + Sync
{
    pub fn new(size: usize, func: F) -> Self {
        let mut rng = thread_rng();
        let side = Uniform::new(-1_f32, 1_f32);
        let state = (0..size).map(|_| rng.sample(side)).collect::<Vec<f32>>();
        Simpleton {
            state,
            func
        }
    }
}

impl<F> Brain for Simpleton<F>
    where F: Fn(f32) -> f32 + Sync
{
    fn evolve(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_idx: usize = rng.gen_range(0, self.state.len());
        let rand_flt: f32   = rng.gen_range(-1_f32, 1_f32);
        self.state[rand_idx] *= rand_flt;
    }
    fn predict_proba(&self, x: &Vec<Vec<f32>>) -> Vec<f32> {
        x
        .par_iter()
        .map(|rec| {
            rec.iter()
                .zip(self.state.iter())
                .map(|(e, s)| *e * *s)
                .sum::<f32>()
        })
        .map(|s| (self.func)(s))
        .collect::<Vec<f32>>()
    }
}