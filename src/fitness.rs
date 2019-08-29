// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::encoding::Binary;
use crate::individual::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
/// Evaluate the fitness of individual in population based on raw_score of
/// individual.
///
/// Fitness is a measure of quality of a solution (individual). A larger value
/// of fitness indicates a better individual in population.
///
/// Fitness evaluation should not be an expensive operation.
///
pub trait EvaluateFitness<G>
where
    G: Genome,
{
    fn evaluate(&self, indvs: &[Individual<G>]) -> Vec<f64>;
}
// base:1 ends here
