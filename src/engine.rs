// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::individual::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
pub struct SimulationStep {
    //
}

pub struct Engine {
    //
}

impl Engine {
    pub fn evolve(&mut self) -> impl Iterator<Item = Result<SimulationStep>> {
        std::iter::from_fn(move || unimplemented!())
    }
}

/// Defines a single step of simulation.
pub trait Simulate {
    //
}
// base:1 ends here
