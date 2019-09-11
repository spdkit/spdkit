// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/operators/mod.rs
// :END:

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mod.rs][mod.rs:1]]
use crate::common::*;
use crate::individual::*;
use crate::population::*;
use crate::random::*;

pub trait GeneticOperator: std::fmt::Debug + Clone {}
impl<T: std::fmt::Debug + Clone> GeneticOperator for T {}

/// For selecting individuals from population.
pub trait SelectionOperator: GeneticOperator {
    /// Select some members from population.
    fn select_from<'a, G, R>(&self, population: &'a Population<G>, rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized;
}

/// For producing new individuals.
pub trait VariationOperator<G>: GeneticOperator
where
    G: Genome,
{
    /// Create genomes for new individuals from the selected.
    fn breed_from<R: Rng + Sized>(&self, parents: &[Member<G>], rng: &mut R) -> Vec<G>;
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

pub mod replacement;
pub mod selection;
pub mod variation;
// mod.rs:1 ends here
