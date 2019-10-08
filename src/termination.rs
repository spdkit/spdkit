// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::individual::*;
use crate::fitness::*;
use crate::population::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
/// Common interface for termination conditions in simulation.
pub trait Terminate {
    fn meets<G: Genome>(&mut self, generation: &Generation<G>) -> bool;
}

// /// Terminates simulation if max allowed evolution generation reached.
// pub struct MaxGeneration(pub usize);

// impl Terminate for MaxGeneration {
//     fn meets<G: Genome>(&mut self, generation: &Generation<G>) -> bool {
//         generation.index >= self.0
//     }
// }
// base:1 ends here

// running mean

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*running%20mean][running mean:1]]
/// Terminates simulation if best solution found so far has no improvement for
/// `n` generations.
///
/// The running mean termination criterion is fulfilled if the difference
/// between the best objective value of the current generation and the average
/// of the best objective values of the last generations is equal to or less
/// than a given threshold `epsilon` (Jainet al., 2001)
#[derive(Debug, Clone)]
pub struct RunningMean {
    // the last n generations for running mean
    nlast: usize,
    epsilon: f64,
    scores: Vec<f64>,
}

impl Default for RunningMean {
    fn default() -> Self {
        Self {
            nlast: 30,
            epsilon: 1e-6,
            scores: Vec::with_capacity(30),
        }
    }
}

impl RunningMean {
    /// Construct a running-mean termination.
    ///
    /// # Parameters
    ///
    /// * nlast: The number of last generations for calculating running average
    /// of best individual fitness.
    ///
    pub fn new(nlast: usize) -> Self {
        Self {
            nlast,
            ..Self::default()
        }
    }
}

impl Terminate for RunningMean {
    fn meets<G: Genome>(&mut self, generation: &Generation<G>) -> bool {
        let best = generation
            .population
            .best_member()
            .expect("empty population!")
            .individual
            .objective_value();

        if generation.index < self.nlast {
            self.scores.push(best);
            return false;
        } else {
            // check if running mean meets termination criterion
            let n = self.scores.len();
            assert_eq!(n, self.nlast);
            let fmean = self.scores.iter().sum::<f64>() / n as f64;
            let diff = (best - fmean).abs();
            if diff < self.epsilon {
                return true;
            }

            // update collected fitness values
            let _ = self.scores.remove(0);
            self.scores.push(best);
        }

        false
    }
}
// running mean:1 ends here

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
        let best = self.best_individual();
        println!(
            " best individual {}: objective value = {:}",
            best.genome(),
            best.objective_value()
        );

        println!("population members:");
        let mut members: Vec<_> = self.population.members().collect();
        members.sort_by_fitness();
        for m in members {
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
