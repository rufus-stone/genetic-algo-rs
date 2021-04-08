use rand::seq::SliceRandom;
use rand::RngCore;

use crate::individual::Individual;

#[derive(Clone, Debug)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, prng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(prng, |individual| individual.fitness())
            .expect("Population is empty!")
    }
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, prng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
