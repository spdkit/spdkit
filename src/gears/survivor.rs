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
    fn survive<R: Rng + Sized>(&mut self, population: &mut Population<G>, rng: &mut R) -> usize;
}

#[derive(Clone)]
pub struct Survivor;

impl<G> Survive<G> for Survivor
where
    G: Genome,
{
    fn survive<R: Rng + Sized>(&mut self, population: &mut Population<G>, _rng: &mut R) -> usize {
        let n_old = population.size();
        population.survive();
        n_old - population.size()
    }
}
// base:1 ends here
