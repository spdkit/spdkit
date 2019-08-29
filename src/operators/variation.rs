// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::encoding::*;
use crate::individual::*;
use crate::population::*;
use crate::random::*;

use super::*;
// imports:1 ends here

// mutation

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mutation][mutation:1]]
#[derive(Debug)]
/// This mutation method simply changes (flips) randomly selected bits.
pub struct FlipBitMutation {
    /// The number of bits to be mutated.
    mutation_size: usize,
}

impl FlipBitMutation {
    pub fn new() -> Self {
        Self { mutation_size: 1 }
    }

    pub fn mutation_size(mut self, n: usize) -> Self {
        self.mutation_size = n;
        self
    }
}

impl<R> VariationOperator<Binary, R> for FlipBitMutation
where
    R: Rng + Sized,
{
    fn breed<T: AsRef<Individual<Binary>>>(&self, indvs: &[T], rng: &mut R) -> Vec<Binary> {
        let mut mutated = vec![];
        for indv in indvs.iter() {
            let mut genome = indv.as_ref().genome().clone();
            let mut choices: Vec<_> = (0..genome.len()).collect();
            let positions = choices.choose_multiple(rng, self.mutation_size).cloned();
            genome.flip(positions);
            mutated.push(genome);
        }

        mutated
    }
}

/// Breed new individuals using clonal selection algorithm.
///
/// # Reference
///
/// Jiao, L.; Wang, L. A Novel Genetic Algorithm Based on Immunity. Systems, Man
/// and Cybernetics, Part A: Systems and Humans, IEEE Transactions on 2000, 30
/// (5), 552–561.
pub struct HyperMutation {
    //
}

#[test]
fn test_mutation() {
    // get global rng
    let mut rng = get_rng!();

    let genomes: Vec<_> = vec!["10111", "01011"]
        .iter()
        .map(|s| Binary::from_str(s))
        .collect();

    let indvs = crate::individual::OneMax.create(genomes);
    for g in FlipBitMutation::new()
        .mutation_size(2)
        .breed(&indvs, &mut *rng)
    {
        // println!("{}", g);
    }
}
// mutation:1 ends here

// crossover

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*crossover][crossover:1]]
#[derive(Debug)]
/// A point on both parents' chromosomes is picked randomly, and designated a
/// 'crossover point'. Bits to the right of that point are swapped between the
/// two parent chromosomes. This results in two offspring, each carrying some
/// genetic information from both parents.
pub struct OnePointCrossOver;

impl<R> VariationOperator<Binary, R> for OnePointCrossOver
where
    R: Rng + Sized,
{
    fn breed<T: AsRef<Individual<Binary>>>(&self, indvs: &[T], rng: &mut R) -> Vec<Binary> {
        assert_eq!(indvs.len(), 2, "only work for two individuals as parents!");
        let mut g1: Binary = indvs[0].as_ref().genome().clone();
        let mut g2: Binary = indvs[1].as_ref().genome().clone();
        let isite = rng.gen_range(0, g1.len());

        for i in 0..isite {
            std::mem::swap(&mut g1[i], &mut g2[i]);
        }

        vec![g1, g2]
    }
}

#[test]
fn test_cx_onepoint() {
    // get global rng
    let mut rng = get_rng!();

    let genomes: Vec<_> = vec!["10111", "01011"]
        .iter()
        .map(|s| Binary::from_str(s))
        .collect();

    let indvs = crate::individual::OneMax.create(genomes);
    for g in OnePointCrossOver.breed(&indvs, &mut *rng) {
        //
    }
}
// crossover:1 ends here
