#![warn(missing_docs)]
//! Creaturae is the intermixing of ML with the classical genetic algorithm.
//! 
//! 

extern crate tensorflow;
extern crate rand;
extern crate num;
extern crate rayon;
extern crate ndarray;

pub mod creature;
pub mod brains;
pub mod population;
pub mod prelude;