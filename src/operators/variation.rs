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

    fn mutate_binary<R: Rng + Sized>(&self, genomes: &mut [Binary], rng: &mut R) {
        for g in genomes.iter_mut() {
           g.mutate(self.mutation_size, rng);
        }
    }
}

impl<R> VariationOperator<Binary, R> for FlipBitMutation
where
    R: Rng + Sized,
{
    fn breed_from(&self, members: &[Member<Binary>], rng: &mut R) -> Vec<Binary> {
        let mut genomes: Vec<_> = members
            .iter()
            .map(|m| m.individual.genome().to_owned())
            .collect();
        self.mutate_binary(&mut genomes, rng);

        genomes
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

impl OnePointCrossOver {
    fn crossover_binary<R: Rng + Sized>(&self, genomes: &[Binary], rng: &mut R) -> Vec<Binary> {
        assert_eq!(genomes.len(), 2, "only work for two genomes as parents!");

        let mut g1 = genomes[0].to_owned();
        let mut g2 = genomes[1].to_owned();
        assert_eq!(g1.len(), g2.len());

        let i = rng.gen_range(0, g1.len());
        std::mem::swap(&mut g1[i], &mut g2[i]);

        vec![g1, g2]
    }
}

impl<R> VariationOperator<Binary, R> for OnePointCrossOver
where
    R: Rng + Sized,
{
    fn breed_from(&self, members: &[Member<Binary>], rng: &mut R) -> Vec<Binary> {
        let genomes: Vec<_> = members
            .iter()
            .map(|m| m.individual.genome().to_owned())
            .collect();

        self.crossover_binary(&genomes, rng)
    }
}

#[test]
fn test_cx_onepoint() {
    use crate::fitness;
    use crate::operators::selection::ElitistSelection;

    // get global rng
    let mut rng = get_rng!();

    let genomes: Vec<_> = vec!["10111", "01011"]
        .iter()
        .map(|s| Binary::from_str(s))
        .collect();

    let indvs = crate::individual::OneMax.create(genomes);
    let population = crate::population::Builder::new(fitness::Maximize).build(indvs);
    let parents = ElitistSelection(2).select_from(&population, &mut *rng);
    for child in OnePointCrossOver.breed_from(&parents, &mut *rng) {
        //
    }
}
// crossover:1 ends here
