//! Contains different [`Brain`] implementations for creatures to use.
//! 
//! 


pub trait Brain {
    fn fit(&mut self, X: Vec<Vec<f32>>, y: Vec<i32>) -> ();
    fn predict(&mut self, X: Vec<Vec<f64>>) -> Vec<i32>;
}
