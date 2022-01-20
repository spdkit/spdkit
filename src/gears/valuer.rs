// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::marker::PhantomData;

use crate::common::*;
use crate::fitness::*;
use crate::individual::*;
use crate::operators::*;
use crate::population::*;

use super::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
pub struct Valuer<G, F, C>
where
    G: Genome,
    F: EvaluateFitness<G>,
    C: EvaluateObjectiveValue<G>,
{
    fitness: Option<F>,
    creator: Option<C>,
    _g: PhantomData<G>,
}

impl<G, F, C> Valuer<G, F, C>
where
    G: Genome,
    F: EvaluateFitness<G>,
    C: EvaluateObjectiveValue<G>,
{
    pub fn new() -> Self {
        Self {
            fitness: None,
            creator: None,
            _g: PhantomData,
        }
    }

    /// Set fitness evaluator for building population.
    pub fn with_fitness(mut self, f: F) -> Self {
        self.fitness = Some(f);
        self
    }

    /// Set evaluator of objective value for creating individuals.
    pub fn with_creator(mut self, c: C) -> Self {
        self.creator = Some(c);
        self
    }

    /// Create individuals from genomes.
    pub fn create_individuals(&self, genomes: Vec<G>) -> Vec<Individual<G>> {
        if let Some(creator) = &self.creator {
            creator.create(genomes)
        } else {
            panic!("creator not set!");
        }
    }

    /// Build a population from individuals.
    pub fn build_population(&mut self, indvs: Vec<Individual<G>>) -> Population<G> {
        if let Some(fitness) = &mut self.fitness {
            Population::build(indvs, fitness)
        } else {
            panic!("fitness not set!");
        }
    }
}
// base:1 ends here
