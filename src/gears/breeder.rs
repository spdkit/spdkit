// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::marker::PhantomData;

use crate::common::*;
use crate::encoding::*;
use crate::individual::*;
use crate::operators::*;
use crate::population::*;
use crate::random::*;

use super::*;
// imports:1 ends here

/// A breeder for genetic algorithm featuring a combined use of crossover and
/// mutation operators.
#[derive(Clone)]
pub struct GeneticBreeder<C, S, G>
where
    C: VariationOperator<G>,
    S: SelectionOperator,
    G: Genome + Mutate,
{
    cx_prob: f64,
    mut_prob: f64,

    crossover: Option<C>,
    selector: Option<S>,
    _g: PhantomData<G>,
}

impl<C, S, G> GeneticBreeder<C, S, G>
where
    C: VariationOperator<G>,
    S: SelectionOperator,
    G: Genome + Mutate,
{
    pub fn new() -> Self {
        Self {
            cx_prob: 1.0,
            mut_prob: 0.1,
            crossover: None,
            selector: None,
            _g: PhantomData,
        }
    }

    pub fn with_crossover(mut self, c: C) -> Self {
        self.crossover = Some(c);
        self
    }

    pub fn with_selector(mut self, s: S) -> Self {
        self.selector = Some(s);
        self
    }

    pub fn crossover_probability(mut self, p: f64) -> Self {
        assert!(p.is_sign_positive());

        self.cx_prob = p;
        self
    }

    pub fn mutation_probability(mut self, p: f64) -> Self {
        assert!(p.is_sign_positive());
        self.mut_prob = p;

        self
    }
}

impl<G, C, S> Breed<G> for GeneticBreeder<C, S, G>
where
    G: Genome + Mutate,
    C: VariationOperator<G>,
    S: SelectionOperator,
{
    /// Breed `m` new genomes from parent population.
    fn breed<R: Rng + Sized>(
        &mut self,
        m: usize,
        population: &Population<G>,
        rng: &mut R,
    ) -> Vec<G> {
        let mut crossover = self.crossover.take().expect("breeder has no crossover");
        let mut selector = self.selector.take().expect("breeder has no selector");

        // loop until required number of genomes
        let mut required_genomes = Vec::with_capacity(m);
        while required_genomes.len() < m {
            let parents = selector.select_from(population, rng);
            let new_genomes = crossover.breed_from(&parents, rng);
            for g in new_genomes {
                required_genomes.push(g);
            }
        }

        // mutate one bit/one point randomly.
        if rng.gen::<f64>() > self.mut_prob {
            for g in required_genomes.iter_mut() {
                g.mutate(1, rng);
            }
        }

        required_genomes
    }
}

// hypermutation

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*hypermutation][hypermutation:1]]

// hypermutation:1 ends here
