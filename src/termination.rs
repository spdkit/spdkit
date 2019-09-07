// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::individual::*;
use crate::fitness::*;
use crate::population::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
pub trait Terminate {
    fn meets<G: Genome>(&mut self, generation: &Generation<G>) -> bool;
}

/// Terminates simulation if max allowed evolution generation reached.
pub struct MaxGeneration(pub usize);

impl Terminate for MaxGeneration {
    fn meets<G: Genome>(&mut self, generation: &Generation<G>) -> bool {
        generation.index >= self.0
    }
}

/// Terminates simulation if best solution found so far has no improvement for
/// `n` generations.
pub struct MaxGenerationNoImprovement {
    n: usize,
}

impl Terminate for MaxGenerationNoImprovement {
    fn meets<G: Genome>(&mut self, generation: &Generation<G>) -> bool {
        unimplemented!()
    }
}
// base:1 ends here

// generation

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*generation][generation:1]]
/// Represents a simulation step during evolution.
#[derive(Debug)]
pub struct Generation<G>
where
    G: Genome,
{
    pub index: usize,
    pub population: Population<G>,
}

impl<G> Generation<G>
where
    G: Genome + std::fmt::Display,
{
    pub fn summary(&self) {
        println!("# generation: {}", self.index);
        println!(
            " best individual raw score = {:}",
            self.best_individual().raw_score()
        );

        println!("population members:");
        for m in self.population.members() {
            println!(" {:}", m);
        }
    }

    /// Return the best individual in this generation.
    pub fn best_individual(&self) -> Individual<G> {
        if let Some(member) = self.population.best_member() {
            member.individual.to_owned()
        } else {
            panic!("empty population!")
        }
    }
}
// generation:1 ends here
