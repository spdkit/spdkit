// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::marker::PhantomData;
// imports:1 ends here

// genome

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*genome][genome:1]]
pub trait Genome: Clone + Send {
    //
}
// genome:1 ends here

// individual

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*individual][individual:1]]
#[derive(Clone, Debug)]
pub struct Individual<G>
where
    G: Genome,
{
    raw_score: f64,
    genome: G,
}

/// Potentially expensive operation.
pub trait Evaluate<G>
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
    pub fn new<E>(genome: G, func: &E) -> Self
    where
        E: Evaluate<G>,
    {
        let raw_score = func.evaluate(&genome);
        Self { genome, raw_score }
    }

    /// Return genome of this individual.
    pub fn genome(&self) -> &G {
        &self.genome
    }

    /// Return the evaluated raw score of this individual.
    pub fn raw_score(&self) -> f64 {
        self.raw_score
    }
}

/// Create individual from genome.
pub struct Creator<E, G>
where
    E: Evaluate<G>,
    G: Genome,
{
    eval_func: E,
    _empty: PhantomData<G>,
}

impl<E, G> Creator<E, G>
where
    E: Evaluate<G>,
    G: Genome,
{
    pub fn new(eval_func: E) -> Self {
        Self {
            eval_func,
            _empty: PhantomData,
        }
    }

    pub fn create(&self, genomes: &[G]) -> Vec<Individual<G>> {
        genomes
            .iter()
            .map(|g| {
                let raw_score = self.eval_func.evaluate(g);
                Individual {
                    genome: g.clone(),
                    raw_score,
                }
            })
            .collect()
    }
}
// individual:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::encoding::Binary;

    impl Genome for Binary {}

    struct OneMax;

    impl Evaluate<Binary> for OneMax {
        fn evaluate(&self, genome: &Binary) -> f64 {
            let s: usize = genome.iter().map(|&b| b as usize).sum();
            s as f64
        }
    }

    #[test]
    fn test() {
        let onemax = OneMax;
        let creator = Creator::new(onemax);
        let keys: Vec<_> = vec!["10110", "01010"]
            .iter()
            .map(|x| Binary::from_str(x))
            .collect();

        let indvs = creator.create(&keys);
        for indv in indvs.iter() {
            println!("indv {:}, raw_score = {}", indv.genome(), indv.raw_score());
        }
    }
}
// test:1 ends here
