// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/gears/mod.rs
// :END:

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mod.rs][mod.rs:1]]
use crate::individual::*;
use crate::population::*;
use crate::random::*;

/// Elemental gear for evolution engine
pub trait Gear<G>
where
    G: Genome,
{
    /// select members from external population
    fn select(&mut self, indvs: &Population<G>);

    /// work on internal population
    fn forward(&mut self);
}

/// Breed `n` new genomes (not-evaluated individual) from parent population.
pub trait Breed<G: Genome>: Clone {
    fn breed<R: Rng + Sized>(
        &mut self,
        n: usize,
        population: &Population<G>,
        rng: &mut R,
    ) -> Vec<G>;
}

mod breeder;
mod valuer;

pub use self::breeder::GeneticBreeder;
pub use self::valuer::Valuer;
// mod.rs:1 ends here
