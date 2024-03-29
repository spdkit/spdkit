// [[file:../spdkit.note::*imports][imports:1]]
use std::marker::PhantomData;

use crate::encoding::*;
use crate::random::*;
// imports:1 ends here

// [[file:../spdkit.note::8469d257][8469d257]]
pub trait Genome: Clone + Send + std::hash::Hash + std::cmp::Eq {
    //
}
// 8469d257 ends here

// [[file:../spdkit.note::edee42d4][edee42d4]]
#[derive(Clone, Debug)]
pub struct Individual<G>
where
    G: Genome,
{
    raw_score: f64,
    genome: G,
}

/// Evaluate the objective value of an individual.
///
/// Potentially expensive operation.
pub trait EvaluateObjectiveValue<G>: Clone + std::fmt::Debug + std::marker::Sync
where
    G: Genome,
{
    fn evaluate(&self, genome: &G) -> f64;
}

impl<G> Individual<G>
where
    G: Genome,
{
    /// Create a new individual from a genome and evaluation function for raw
    /// score.
    pub fn new<E>(genome: G, func: &mut E) -> Self
    where
        E: EvaluateObjectiveValue<G>,
    {
        let raw_score = func.evaluate(&genome);
        Self { genome, raw_score }
    }

    /// Return genome of this individual.
    pub fn genome(&self) -> &G {
        &self.genome
    }

    /// Return the evaluated objective value of this individual.
    ///
    /// This is sometimes referred to as objective fitness since this
    /// measurement is based solely on an individual's geno/phenotype and is not
    /// affected by other factors such as the current makeup of the population
    ///
    /// # Reference
    ///
    /// * De Jong 2006
    ///
    pub fn objective_value(&self) -> f64 {
        self.raw_score
    }
}

impl<G> AsRef<Individual<G>> for Individual<G>
where
    G: Genome,
{
    fn as_ref(&self) -> &Self {
        self
    }
}
// edee42d4 ends here

// [[file:../spdkit.note::ad65eec7][ad65eec7]]
use gut::prelude::*;

/// blanket implementation for creating new individuals from genomes
pub(crate) trait Create<G>
where
    G: Genome,
{
    fn create(&self, genomes: impl IntoIterator<Item = G>) -> Vec<Individual<G>>;
}

impl<G, T> Create<G> for T
where
    G: Genome,
    T: EvaluateObjectiveValue<G>,
{
    /// Create individuals from genomes.
    fn create(&self, genomes: impl IntoIterator<Item = G>) -> Vec<Individual<G>> {
        // remove possible duplicates
        let genomes: Vec<_> = genomes.into_iter().collect();
        let n = genomes.len();
        let genomes: std::collections::HashSet<_> = genomes.into_iter().collect();
        let m = n - genomes.len();
        if m > 0 {
            info!("Removed {m} duplicates from {n} created genomes.");
        }

        // 2022-01-20: allow run in parallel
        genomes
            .into_par_iter()
            .map(|g| {
                let raw_score = self.evaluate(&g);
                Individual { genome: g, raw_score }
            })
            .collect()
    }
}
// ad65eec7 ends here

// [[file:../spdkit.note::*onemax][onemax:1]]
#[derive(Clone, Debug)]
pub struct OneMax;

impl EvaluateObjectiveValue<Binary> for OneMax {
    fn evaluate(&self, genome: &Binary) -> f64 {
        let s: usize = genome.iter().filter(|&b| *b).count();
        s as f64
    }
}
// onemax:1 ends here

// [[file:../spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::encoding::Binary;

    #[test]
    fn test() {
        let codes: Vec<_> = vec!["10110", "01010"]
            .iter()
            .map(|x| Binary::from_str(x))
            .collect();

        let indvs = OneMax.create(codes);
        for indv in indvs.iter() {
            println!(
                "indv {:}, objective value = {}",
                indv.genome(),
                indv.objective_value()
            );
        }
    }
}
// test:1 ends here
