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
#[derive(Debug, Clone)]
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

impl VariationOperator<Binary> for FlipBitMutation {
    fn breed_from<R: Rng + Sized>(&self, members: &[Member<Binary>], rng: &mut R) -> Vec<Binary> {
        let mut genomes: Vec<_> = members
            .iter()
            .map(|m| m.genome().to_owned())
            .collect();
        self.mutate_binary(&mut genomes, rng);

        genomes
    }
}
// mutation:1 ends here

// onepoint crossover

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*onepoint%20crossover][onepoint crossover:1]]
/// A point on both parents' chromosomes is picked randomly, and designated a
/// 'crossover point'. Bits to the right of that point are swapped between the
/// two parent chromosomes. This results in two offspring, each carrying some
/// genetic information from both parents.
#[derive(Debug, Clone)]
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

impl VariationOperator<Binary> for OnePointCrossOver {
    fn breed_from<R: Rng + Sized>(&self, members: &[Member<Binary>], rng: &mut R) -> Vec<Binary> {
        let genomes: Vec<_> = members.iter().map(|m| m.genome().to_owned()).collect();

        self.crossover_binary(&genomes, rng)
    }
}

#[test]
fn test_cx_onepoint() {
    use crate::operators::selection::ElitistSelection;

    // get global rng
    let mut rng = get_rng!();

    let genomes: Vec<_> = vec!["10111", "01011"]
        .iter()
        .map(|s| Binary::from_str(s))
        .collect();

    let indvs = crate::individual::OneMax.create(genomes);
    let mut fitness = crate::fitness::Maximize;
    let population = Population::build(indvs, &mut fitness);
    let parents = ElitistSelection::new(2).select_from(&population, &mut *rng);
    for child in OnePointCrossOver.breed_from(&parents, &mut *rng) {
        //
    }
}
// onepoint crossover:1 ends here

// triadic crossover

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*triadic%20crossover][triadic crossover:1]]
#[derive(Debug, Clone)]
pub struct TriadicCrossOver;

impl TriadicCrossOver {
    fn crossover<R: Rng + Sized>(&self, members: &[Member<Binary>], rng: &mut R) -> Vec<Binary> {
        debug!("breed new individuals using {} members.", members.len());

        // sort by energy from lowest to highest
        let mut members = members.to_vec();
        members.sort_by_fitness();

        for m in members.iter() {
            debug!(">> {}", m);
        }

        let parent0 = members[0].individual.genome();
        let parent1 = members[1].individual.genome();
        let parent2 = members[2].individual.genome();

        let positions_swap: Vec<_> = parent0
            .iter()
            .zip(parent1.iter())
            .enumerate()
            .filter_map(|(i, (b1, b2))| if b1 == b2 { Some(i) } else { None })
            .collect();

        let mut child1 = parent1.to_owned();
        let mut child2 = parent2.to_owned();
        for i in positions_swap {
            std::mem::swap(&mut child1[i], &mut child2[i]);
        }

        vec![child1, child2]
    }
}

impl VariationOperator<Binary> for TriadicCrossOver {
    fn breed_from<R: Rng + Sized>(&self, parents: &[Member<Binary>], rng: &mut R) -> Vec<Binary> {
        self.crossover(&parents, rng)
    }
}
// triadic crossover:1 ends here
