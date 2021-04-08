use rand::{Rng, RngCore};

use crate::chromosome::Chromosome;

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// 0.0 = no genes will be touched
    /// 1.0 = all genes will be touched
    chance: f32,

    /// Magnitude of that change:
    /// 0.0 = touched genes will not be modified
    /// 3.0 = touched genes will be += or -= by at most 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, prng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if prng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if prng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * prng.gen::<f32>();
            }
        }
    }
}

pub trait MutationMethod {
    fn mutate(&self, prng: &mut dyn RngCore, child: &mut Chromosome);
}
