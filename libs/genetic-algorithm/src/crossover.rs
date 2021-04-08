use rand::{Rng, RngCore};

use crate::chromosome::Chromosome;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        prng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        prng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let mut child = Vec::new();
        let gene_count = parent_a.len();

        for gene_idx in 0..gene_count {
            let gene = if prng.gen_bool(0.5) {
                parent_a[gene_idx]
            } else {
                parent_b[gene_idx]
            };

            child.push(gene);
        }

        child.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn crossover() {
        let mut prng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover::new().crossover(&mut prng, &parent_a, &parent_b);

        // Number of genes different between `child` and `parent_a`
        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();

        // Number of genes different between `child` and `parent_b`
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

        // Roughly looks like 50%, which proves that chance for picking either
        // gene *is* 50%
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
