// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::individual::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
/// Evaluate fitnesses of individuals. A larger value of fitness indicates a
/// better individual in population.
pub trait EvaluateFitness<G>
where
    G: Genome,
{
    fn evaluate(&self, indvs: &[Individual<G>]) -> Vec<f64>;
}
// base:1 ends here
