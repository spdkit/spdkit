// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::individual::*;
use crate::fitness::*;
use crate::population::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
/// Represents a simulation step during evolution.
#[derive(Debug)]
pub struct Generation<G>
where
    G: Genome,
{
    igeneration: usize,
    population: Population<G>,
}

impl<G> Generation<G>
where
    G: Genome + std::fmt::Display,
{
    pub fn summary(&self) {
        println!("# generation: {}", self.igeneration);

        for m in self.population.members() {
            println!(" {:}", m);
        }
    }
}
// base:1 ends here

// engine

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*engine][engine:1]]
use crate::encoding::Binary;
use crate::operators::*;
use crate::random::*;
use std::iter::FromIterator;

/// Evolution engine.
pub struct Engine {
    population_size_limit: usize,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            population_size_limit: 10,
        }
    }
}

struct FitnessMax;

impl EvaluateFitness<Binary> for FitnessMax {
    fn evaluate(&self, indvs: &[Individual<Binary>]) -> Vec<f64> {
        indvs.iter().map(|x| x.raw_score()).collect()
    }
}

impl Engine {
    pub fn evolve(&mut self) -> impl Iterator<Item = Result<Generation<Binary>>> {
        let n_limit = self.population_size_limit;
        let mut cur_population = build_initial_population(n_limit);

        let selector = crate::operators::selection::ElitistSelection(2);
        let crossover = crate::operators::variation::OnePointCrossOver;
        let mutator = crate::operators::variation::FlipBitMutation::new();

        let mut rng = get_rng!();

        let mut ig = 0;
        std::iter::from_fn(move || {
            let new_population = if ig == 0 {
                println!("initial population:");
                for m in cur_population.members() {
                    println!("{}", m);
                }
                cur_population.clone()
            } else {
                // 1. breed new individuals from old population
                let indvs = selector.select_from(&cur_population, &mut *rng);
                println!("Selected {} individuals", indvs.len());
                let new_genomes = crossover.breed(&indvs, &mut *rng);
                let mut new_indvs = OneMax.create(new_genomes);
                println!("bred {} new individuals", new_indvs.len());

                // 2. create new population by supplanting bad performing individuals

                // 2.1 combine all available individuals into one.
                let old_indvs = cur_population.individuals();
                new_indvs.extend_from_slice(old_indvs);

                // 2.2 create a new population from combined new individuals
                let mut new_population = crate::population::Builder::new(FitnessMax)
                    .size_limit(n_limit)
                    .build(new_indvs);

                // 2.3 remove `n` some bad performing individuals
                let n = new_population.survive();
                println!("removed {} bad individuals.", n);

                // 2.4 update current population.
                std::mem::swap(&mut cur_population, &mut new_population);
                cur_population.clone()
            };

            let g = Generation {
                igeneration: ig,
                population: new_population,
            };

            ig += 1;

            Some(Ok(g))
        })
    }
}

// test only
fn build_initial_population(n: usize) -> Population<Binary> {
    // generate `n` binary genomes in size of 10.
    let keys: Vec<_> = (0..n).map(|_| random_binary(11)).collect();

    let indvs = crate::individual::OneMax.create(keys);

    crate::population::Builder::new(FitnessMax).build(indvs)
}

fn random_binary(length: usize) -> Binary {
    let mut rng = get_rng!();
    let list: Vec<_> = (0..length).map(|_| rng.gen()).collect();
    Binary::new(list)
}
// engine:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::common::*;

    #[test]
    fn test_engine() -> Result<()> {
        let mut engine = Engine::default();
        for g in engine.evolve().take(10) {
            let generation = g?;
            generation.summary();
        }

        Ok(())
    }
}
// test:1 ends here
