// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::encoding::*;
use crate::individual::*;
use crate::operators::*;
use crate::population::*;
use crate::random::*;

use super::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
/// Member supplanting by removing bad performing individuals.
pub trait Survive<G: Genome>: Clone {
    fn survive<R: Rng + Sized>(
        &mut self,
        population: Population<G>,
        rng: &mut R,
    ) -> Vec<Individual<G>>;
}

#[derive(Clone)]
pub struct Survivor;

impl<G> Survive<G> for Survivor
where
    G: Genome,
{
    fn survive<R: Rng + Sized>(
        &mut self,
        population: Population<G>,
        rng: &mut R,
    ) -> Vec<Individual<G>> {
        let mut members: Vec<_> = population.members().collect();
        members.sort_by_fitness();
        members
            .into_iter()
            .take(population.size_limit())
            .map(|m| m.individual.to_owned())
            .collect()
    }
}
// base:1 ends here
