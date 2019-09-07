// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/operators/mod.rs
// :END:

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mod.rs][mod.rs:1]]
use crate::common::*;
use crate::individual::*;
use crate::population::*;
use crate::random::*;

pub trait GeneticOperator {}
impl<T: std::fmt::Debug> GeneticOperator for T {}

/// For selecting individuals from population.
pub trait SelectionOperator<'a>: GeneticOperator {
    /// Select some members from population.
    fn select_from<G, R>(&self, population: &'a Population<G>, rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized;
}

/// For individual replacement
pub trait ReplacementOperator: GeneticOperator {
    /// Remove `n` bad performaing members in population.
    fn remove_from<G: Genome, R: Rng + Sized>(
        &self,
        n: usize,
        population: &mut Population<G>,
        rng: &mut R,
    );
}

/// For producing new individuals.
pub trait VariationOperator<'a, G, R>: GeneticOperator
where
    G: Genome,
    R: Rng + Sized,
{
    /// Create genomes for new individuals from the selected.
    fn breed_from(&self, parents: &[Member<'a, G>], rng: &mut R) -> Vec<G>;
}

pub mod replacement;
pub mod selection;
pub mod variation;
// mod.rs:1 ends here
