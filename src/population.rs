// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::marker::PhantomData;

use crate::fitness::*;
use crate::individual::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
/// A population is a collection of evaluated individuals (fitness).
#[derive(Clone, Debug)]
pub struct Population<G>
where
    G: Genome,
{
    /// The individuals in this population.
    individuals: Vec<Individual<G>>,

    /// The max number of individuals allowed to be survived in this population.
    size_limit: usize,

    /// The fitness values of all individuals.
    fitness_values: Vec<f64>,
}

impl<G> Population<G>
where
    G: Genome,
{
    /// Re-evaluate individuals in population with `fitness` function.
    pub fn evaluate_with<F: EvaluateFitness<G>>(&mut self, fitness: F) {
        self.fitness_values = evaluate_individuals(&self.individuals, &fitness);
    }

    /// Re-evaluate individual fitness with weight.
    pub fn weight_with(&mut self, weight: f64) {
        for fitness in self.fitness_values.iter_mut() {
            *fitness *= weight;
        }
    }

    /// Return a member view of individuals with associated fitness values.
    pub fn members(&self) -> impl Iterator<Item = Member<G>> {
        self.individuals
            .iter()
            .zip(self.fitness_values.iter())
            .map(|(individual, fitness)| Member {
                individual,
                fitness,
            })
    }

    /// Return a list of individuals in this population.
    pub fn individuals(&self) -> &[Individual<G>] {
        &self.individuals
    }

    /// Return population size.
    pub fn size(&self) -> usize {
        self.individuals.len()
    }

    /// Set population size limit.
    pub fn set_size_limit(&mut self, limit: usize) {
        self.size_limit = limit;
    }

    /// Return population size limit.
    pub fn size_limit(&self) -> usize {
        self.size_limit
    }

    /// Return true of there are too many individuals in this population.
    pub fn is_oversized(&self) -> bool {
        self.size() > self.size_limit
    }
}

/// Population builder.
pub struct Builder<G, F>
where
    G: Genome,
    F: EvaluateFitness<G>,
{
    fitness: F,
    _empty: PhantomData<G>,
}

impl<G, F> Builder<G, F>
where
    G: Genome,
    F: EvaluateFitness<G>,
{
    /// Construct a population builder with a fitness function.
    pub fn new(fitness: F) -> Self {
        Self {
            fitness,
            _empty: PhantomData,
        }
    }

    /// Build a population from a group of individuals.
    pub fn build(&self, indvs: Vec<Individual<G>>) -> Population<G> {
        let fitness_values = evaluate_individuals(&indvs, &self.fitness);

        Population {
            individuals: indvs,
            size_limit: 10,
            fitness_values,
        }
    }
}

/// Evaluate individuals with a fitness function.
fn evaluate_individuals<G, F>(indvs: &[Individual<G>], fitness: &F) -> Vec<f64>
where
    G: Genome,
    F: EvaluateFitness<G>,
{
    let fitness_values = fitness.evaluate(indvs);
    assert_eq!(
        fitness_values.len(),
        indvs.len(),
        "fitness values is not equal to the number of individuals!"
    );
    fitness_values
}
// base:1 ends here

// member view
// sort members in the order of best fitness first.

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*member%20view][member view:1]]
/// A member is a view of individual with its fitness in parent Population.
pub struct Member<'a, G>
where
    G: Genome,
{
    pub individual: &'a Individual<G>,
    pub fitness: &'a f64,
}

pub trait SortMember {
    fn sort_by_fitness(&mut self);
}

impl<'a, G> SortMember for [Member<'a, G>]
where
    G: Genome,
{
    fn sort_by_fitness(&mut self) {
        self.sort_by(|mi, mj| {
            let (fi, fj) = (mi.fitness, mj.fitness);
            // ignore NaN items
            fj.partial_cmp(fi).unwrap_or(std::cmp::Ordering::Less)
        });
    }
}
// member view:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::encoding::Binary;

    struct OneMax;

    impl Evaluate<Binary> for OneMax {
        fn evaluate(&self, genome: &Binary) -> f64 {
            let s: usize = genome.iter().map(|&b| b as usize).sum();
            s as f64
        }
    }

    struct FitnessMax;

    impl EvaluateFitness<Binary> for FitnessMax {
        fn evaluate(&self, indvs: &[Individual<Binary>]) -> Vec<f64> {
            indvs.iter().map(|x| x.raw_score()).collect()
        }
    }

    #[test]
    fn test() {
        let onemax = OneMax;
        let creator = crate::individual::Creator::new(onemax);
        let keys: Vec<_> = vec!["10110", "01010", "11011"]
            .iter()
            .map(|x| Binary::from_str(x))
            .collect();

        let indvs = creator.create(&keys);
        let pop = crate::population::Builder::new(FitnessMax).build(indvs);
        let mut members: Vec<_> = pop.members().collect();
        members.sort_by_fitness();

        for m in members.iter() {
            println!(
                "indv {}, raw_score: {}, fitness = {}",
                m.individual.genome(),
                m.individual.raw_score(),
                m.fitness
            );
        }
    }
}
// test:1 ends here
