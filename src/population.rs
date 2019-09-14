// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::marker::PhantomData;

use crate::common::*;
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

    /// Re-evaluate individuals in population with `fitness` function.
    pub fn evaluate_with<F: EvaluateFitness<G>>(&mut self, fitness: F) {
        self.fitness_values = evaluate_individuals(&self.individuals, &fitness);
    }

    /// Re-evaluate individual fitness with individual weight.
    pub fn weight_with(&mut self, weight: f64) {
        for fitness in self.fitness_values.iter_mut() {
            *fitness *= weight;
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

// builder

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*builder][builder:1]]
/// Population builder.
pub struct Builder<G, F>
where
    G: Genome,
    F: EvaluateFitness<G>,
{
    fitness: F,
    size_limit: usize,
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
            size_limit: 10,
            _empty: PhantomData,
        }
    }

    /// Constraint population size.
    pub fn size_limit(mut self, n: usize) -> Self {
        self.size_limit = n;
        self
    }

    /// Build a population from a group of individuals.
    pub fn build(&self, indvs: Vec<Individual<G>>) -> Population<G> {
        let fitness_values = evaluate_individuals(&indvs, &self.fitness);

        Population {
            individuals: indvs,
            fitness_values,
            size_limit: self.size_limit,
        }
    }
}
// builder:1 ends here

// members
// sort members in the order of best fitness first.

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*members][members:1]]
/// A member is a view of individual with its fitness in parent Population.
#[derive(Debug, Clone)]
pub struct Member<'a, G>
where
    G: Genome,
{
    pub individual: &'a Individual<G>,
    pub fitness: f64,
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
            // puts NANs at the end
            float_ordering_maximize(&fi, &fj)
        });
    }
}

/// Nice output for the Genome implemeting Display trait.
impl<'a, G> std::fmt::Display for Member<'a, G>
where
    G: Genome + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "indv {}, raw_score: {}, fitness = {}",
            self.individual.genome(),
            self.individual.objective_value(),
            self.fitness
        )
    }
}

impl<G> Population<G>
where
    G: Genome,
{
    /// Return a member view of individuals with associated fitness values.
    pub fn members(&self) -> impl Iterator<Item = Member<G>> {
        self.individuals
            .iter()
            .zip(self.fitness_values.iter())
            .map(|(individual, &fitness)| Member {
                individual,
                fitness,
            })
    }

    /// Return a member view of the best individual in this population.
    pub fn best_member(&self) -> Option<Member<G>> {
        self.fitness_values.iter().imax().and_then(|(i, fitness)| {
            let best = Member {
                individual: &self.individuals[i],
                fitness,
            };
            Some(best)
        })
    }
}
// members:1 ends here

// survive

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*survive][survive:1]]
impl<G> Population<G>
where
    G: Genome,
{
    /// Remove some bad performing individuals to fit the population size limit
    /// constrain.
    ///
    /// # Returns
    ///
    /// * return the number of individuals to be removed.
    ///
    pub fn survive(&mut self) -> usize {
        if self.is_oversized() {
            let n_remove = self.size() - self.size_limit;
            let mut members: Vec<_> = self.members().collect();
            members.sort_by_fitness();

            let mut indvs = vec![];
            let mut values = vec![];
            for m in members.into_iter().take(self.size_limit) {
                indvs.push(m.individual.to_owned());
                values.push(m.fitness);
            }

            self.individuals = indvs;
            self.fitness_values = values;
            n_remove
        } else {
            0
        }
    }

    /// Return true of there are too many individuals in this population.
    pub fn is_oversized(&self) -> bool {
        self.size() > self.size_limit
    }
}
// survive:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::encoding::Binary;

    #[test]
    fn test() {
        let keys: Vec<_> = vec!["10110", "01010", "11011"]
            .iter()
            .map(|x| Binary::from_str(x))
            .collect();

        let indvs = crate::individual::OneMax.create(keys);
        let pop = crate::population::Builder::new(crate::fitness::Maximize)
            .size_limit(10)
            .build(indvs);
        assert!(!pop.is_oversized());

        let mut members: Vec<_> = pop.members().collect();
        members.sort_by_fitness();

        for m in members.iter() {
            // dbg!(m.individual, m.fitness);
        }

        let m = pop.best_member().unwrap();
        assert_eq!(m.individual.genome().to_string(), "11011");
    }
}
// test:1 ends here
