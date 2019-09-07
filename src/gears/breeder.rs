// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::encoding::*;
use crate::individual::*;
use crate::operators::*;
use crate::population::*;
use crate::random::*;

use super::*;

/// Breed new individuals from parent population.
pub trait Breed<G: Genome> {
    // fn select(&mut self, population: &Population<G>);

    fn breed(&mut self) -> Vec<Individual<G>>;
}
// imports:1 ends here

// hypermutation

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*hypermutation][hypermutation:1]]

// hypermutation:1 ends here
