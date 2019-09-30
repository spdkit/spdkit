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
/// Member surviving by removing bad performing individuals
pub trait Survive<G: Genome>: Clone {
    fn survive<R: Rng + Sized>(&mut self, population: &Population<G>, rng: &mut R) -> usize;
}
// base:1 ends here
