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
pub struct Survivor {
    /// Whether to remove duplicates in population.
    remove_duplicates: bool,
}

impl Survivor {
    pub fn create() -> Self {
        Self::default()
    }

    pub fn remove_duplicates(mut self, r: bool) -> Self {
        self.remove_duplicates = r;
        self
    }
}

impl Default for Survivor {
    fn default() -> Self {
        Self {
            remove_duplicates: false,
        }
    }
}

impl<G> Survive<G> for Survivor
where
    G: Genome + Ord,
{
    fn survive<R: Rng + Sized>(
        &mut self,
        population: Population<G>,
        rng: &mut R,
    ) -> Vec<Individual<G>> {
        let mut members: Vec<_> = population.members().collect();
        if self.remove_duplicates {
            members.sort_by(|a, b| a.genome().cmp(b.genome()));
            members.dedup_by(|a, b| a.genome() == b.genome());
        }

        members.sort_by_fitness();
        members
            .into_iter()
            .take(population.size_limit())
            .map(|m| m.individual.to_owned())
            .collect()
    }
}
// base:1 ends here
