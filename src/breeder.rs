// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::individual::*;
use crate::operator::*;
use crate::population::*;
use crate::random::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
pub trait BreedingOperator<G, R>
where
    G: Genome,
    R: Rng + Sized,
{
    /// Create new genomes from selected individuals.
    fn breed(&self, indvs: &[Individual<G>], rng: &mut R) -> Vec<G>;
}
// base:1 ends here

// mutation

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mutation][mutation:1]]
struct Mutation {
    //
}

impl<G, R> BreedingOperator<G, R> for Mutation
where
    G: Genome,
    R: Rng + Sized,
{
    fn breed(&self, indvs: &[Individual<G>], rng: &mut R) -> Vec<G>
    {
        unimplemented!()
    }
}
// mutation:1 ends here
