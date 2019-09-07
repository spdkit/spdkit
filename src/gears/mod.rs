// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/gears/mod.rs
// :END:

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mod.rs][mod.rs:1]]
use crate::individual::*;
use crate::population::*;

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

pub mod breeder;
// mod.rs:1 ends here
