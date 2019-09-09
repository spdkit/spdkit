// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::iter::FromIterator;

use crate::common::*;
use crate::encoding::*;
use crate::fitness::*;
use crate::individual::*;
use crate::operators::*;
use crate::population::*;
use crate::random::*;
use crate::termination::*;
// imports:1 ends here

// engine

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*engine][engine:1]]
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

impl Engine {
    /// Evolves one step forward.
    ///
    /// # Returns
    ///
    /// * return an iterator over `Generation`.
    ///
    pub fn evolve(&mut self) -> impl Iterator<Item = Result<Generation<Binary>>> {
        let n_limit = self.population_size_limit;
        let mut cur_population = build_initial_population(n_limit);

        let selector = crate::operators::selection::ElitistSelection(2);
        let crossover = crate::operators::variation::OnePointCrossOver;

        let mut rng = get_rng!();

        let mut ig = 0;
        let mut_prob = 0.1;
        std::iter::from_fn(move || {
            let new_population = if ig == 0 {
                println!("initial population:");
                for m in cur_population.members() {
                    println!("{}", m);
                }
                cur_population.clone()
            } else {
                // 1. breed new individuals from old population
                let parents = selector.select_from(&cur_population, &mut *rng);
                println!("Selected {} members as parents", parents.len());
                let mut new_genomes = crossover.breed_from(&parents, &mut *rng);

                // mutate one bit randomly.
                if rng.gen::<f64>() > mut_prob {
                    for g in new_genomes.iter_mut() {
                        g.mutate(1, &mut *rng);
                    }
                }

                let mut new_indvs = OneMax.create(new_genomes);
                println!("bred {} new individuals", new_indvs.len());

                // 2. create new population by supplanting bad performing individuals

                // 2.1 combine all available individuals into one.
                let old_indvs = cur_population.individuals();
                new_indvs.extend_from_slice(old_indvs);

                // 2.2 create a new population from combined new individuals
                let mut new_population = crate::population::Builder::new(Maximize)
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
                index: ig,
                population: new_population,
            };

            ig += 1;

            Some(Ok(g))
        })
    }

    /// Run the simulation until termination conditions met.
    ///
    /// # Returns
    ///
    /// * return the final `Generation`.
    ///
    pub fn run_until<T: Terminate>(
        &mut self,
        conditions: impl IntoIterator<Item = T>,
    ) -> Result<Generation<Binary>> {
        let mut conditions: Vec<_> = conditions.into_iter().collect();
        for g in self.evolve() {
            let generation = g?;
            for t in conditions.iter_mut() {
                if t.meets(&generation) {
                    return Ok(generation);
                }
            }
        }
        unreachable!()
    }
}

// test only
fn build_initial_population(n: usize) -> Population<Binary> {
    // generate `n` binary genomes in size of 10.
    let keys: Vec<_> = (0..n).map(|_| random_binary(11)).collect();

    let indvs = crate::individual::OneMax.create(keys);

    crate::population::Builder::new(Maximize).build(indvs)
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
