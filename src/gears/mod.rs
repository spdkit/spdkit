// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/gears/mod.rs
// :END:

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mod.rs][mod.rs:1]]
use crate::individual::*;
use crate::population::*;

/// Elemental gear in evolution engine
pub trait Gear {
    /// select members from external population
    fn select<G: Genome>(&self, indvs: &Population<G>);

    /// work on internal population
    fn work();
}
// mod.rs:1 ends here
