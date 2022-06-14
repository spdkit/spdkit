// [[file:../spdkit.note::*imports][imports:1]]
use std::marker::PhantomData;

use crate::common::*;
use crate::fitness::*;
use crate::individual::*;
// imports:1 ends here

// [[file:../spdkit.note::*base][base:1]]
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

    /// Return true of there are too many individuals in this population.
    pub fn is_oversized(&self) -> bool {
        self.size() > self.size_limit
    }

    /// Return population size limit.
    pub fn size_limit(&self) -> usize {
        self.size_limit
    }

    /// Re-evaluate individuals in population with `fitness` function.
    pub fn evaluate_with<F: EvaluateFitness<G>>(&mut self, fitness: &mut F) {
        self.fitness_values = evaluate_individuals(&self.individuals, fitness);
    }

    /// Re-evaluate individual fitness with individual weight.
    pub fn weight_with(&mut self, weight: f64) {
        for fitness in self.fitness_values.iter_mut() {
            *fitness *= weight;
        }
    }

    /// Build a population from individuals `indvs` and fitness function.
    /// The population size limit is set as the size of `indvs`.
    pub fn build<F>(indvs: Vec<Individual<G>>, fitness: &mut F) -> Self
    where
        F: EvaluateFitness<G>,
    {
        let nlimit = indvs.len();
        let fitness_values = evaluate_individuals(&indvs, fitness);

        Self {
            individuals: indvs,
            fitness_values,
            size_limit: nlimit,
        }
    }

    /// Construct with population size limit.
    pub fn with_size_limit(mut self, limit: usize) -> Self {
        self.size_limit = limit;
        self
    }
}

/// Evaluate individuals with a fitness function.
fn evaluate_individuals<G, F>(indvs: &[Individual<G>], fitness: &mut F) -> Vec<f64>
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

// [[file:../spdkit.note::*members][members:1]]
/// A member is a view of individual with its evaluated fitness value in parent
/// Population.
#[derive(Debug, Clone)]
pub struct Member<'a, G>
where
    G: Genome,
{
    pub individual: &'a Individual<G>,
    fitness: f64,
}

pub trait SortMember {
    fn sort_by_fitness(&mut self);
}

impl<'a, G> Member<'a, G>
where
    G: Genome,
{
    /// Return individual objective value.
    pub fn objective_value(&self) -> f64 {
        self.individual.objective_value()
    }

    /// Return individual fitness value.
    pub fn fitness_value(&self) -> f64 {
        self.fitness
    }

    /// Return a reference to individual genome.
    pub fn genome(&self) -> &G {
        self.individual.genome()
    }
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
            "indv {}: fitness = {:5.2} objective value = {}",
            self.individual.genome(),
            self.fitness,
            self.individual.objective_value(),
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
            .map(|(individual, &fitness)| Member { individual, fitness })
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

// [[file:../spdkit.note::*survive][survive:1]]
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
            let n_old = self.size();
            let mut members: Vec<_> = self.members().collect();
            members.sort_by_fitness();
            let to_keep: Vec<_> = members.into_iter().take(self.size_limit).collect();

            let mut indvs = vec![];
            let mut values = vec![];
            for m in to_keep {
                indvs.push(m.individual.to_owned());
                values.push(m.fitness_value());
            }

            self.individuals = indvs;
            self.fitness_values = values;
            n_old - self.size()
        } else {
            0
        }
    }
}
// survive:1 ends here

// [[file:../spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::encoding::Binary;

    #[test]
    fn test() {
        let keys: Vec<_> = vec!["10110", "01010", "11011"].iter().map(|x| Binary::from_str(x)).collect();

        let mut fitness = crate::fitness::Maximize;
        let indvs = crate::individual::OneMax.create(keys);
        let pop = Population::build(indvs, &mut fitness).with_size_limit(10);

        assert!(!pop.is_oversized());

        let mut members: Vec<_> = pop.members().collect();
        members.sort_by_fitness();

        for m in members.iter() {
            // dbg!(m.individual, m.fitness_value());
        }

        let m = pop.best_member().unwrap();
        assert_eq!(m.genome().to_string(), "11011");
    }
}
// test:1 ends here
