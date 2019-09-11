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
use crate::gears::*;
// imports:1 ends here

// core

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*core][core:1]]
pub(crate) fn evolve_one_step<C, B, F, R>(
    cur_population: &Population<Binary>,
    mut creator: C,
    mut breeder: B,
    fitness: F,
    rng: &mut R,
) -> Population<Binary>
where
    C: EvaluateScore<Binary>,
    B: Breed<Binary>,
    F: EvaluateFitness<Binary>,
    R: Rng + Sized,
{
    // 1. create m new individuals from parent population.
    let m = cur_population.size();
    let n_limit = cur_population.size_limit();
    let new_genomes = breeder.breed(m, cur_population, rng);
    let mut new_indvs = creator.create(new_genomes);
    println!("bred {} new individuals", new_indvs.len());

    // 2. create new population by supplanting bad performing individuals

    // 2.1 combine all available individuals into one.
    let old_indvs = cur_population.individuals();
    new_indvs.extend_from_slice(old_indvs);

    // 2.2 create a new population from combined new individuals
    let mut new_population = crate::population::Builder::new(fitness)
        .size_limit(n_limit)
        .build(new_indvs);

    // 2.3 remove `n` some bad performing individuals
    let n = new_population.survive();
    println!("removed {} bad individuals.", n);
    new_population.to_owned()
}

// test only
pub(crate) fn build_initial_population(n: usize) -> Population<Binary> {
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
// core:1 ends here

// pub

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*pub][pub:1]]
use std::marker::PhantomData;

/// Evolution engine.
pub struct Engine<C, F, B>
where
    C: EvaluateScore<Binary>,
    B: Breed<Binary>,
    F: EvaluateFitness<Binary>,
{
    population_size_limit: usize,
    population: Population<Binary>,

    mut_prob: f64,

    creator: Option<C>,
    breeder: Option<B>,
    fitness: Option<F>,
}

impl<C, F, B> Engine<C, F, B>
where
    C: EvaluateScore<Binary>,
    B: Breed<Binary>,
    F: EvaluateFitness<Binary>,
{
    pub fn new(initial_population: Population<Binary>) -> Self {
        Self {
            population: initial_population,
            population_size_limit: 10,
            mut_prob: 0.1,
            creator: None,
            breeder: None,
            fitness: None,
        }
    }

    pub fn with_fitness(mut self, f: F) -> Self {
        self.fitness = Some(f);
        self
    }

    pub fn with_creator(mut self, c: C) -> Self {
        self.creator = Some(c);
        self
    }

    pub fn with_breeder(mut self, b: B) -> Self {
        self.breeder = Some(b);
        self
    }

    /// Evolves one step forward.
    ///
    /// # Returns
    ///
    /// * return an iterator over `Generation`.
    ///
    pub fn evolve<'a>(&'a mut self) -> impl Iterator<Item = Result<Generation<Binary>>> + 'a {
        let mut rng = get_rng!();
        let mut ig = 0;

        let creator = self.creator.take().expect("no creator");
        let fitness = self.fitness.take().expect("no fitness");
        let breeder = self.breeder.take().expect("no breeder");

        std::iter::from_fn(move || {
            if ig == 0 {
                println!("initial population:");
                for m in self.population.members() {
                    println!("{}", m);
                }
            } else {
                let mut new_population = evolve_one_step(
                    &self.population,
                    creator.clone(),
                    breeder.clone(),
                    fitness.clone(),
                    &mut *rng,
                );
                self.population = new_population;
            }

            let g = Generation {
                index: ig,
                population: self.population.clone(),
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
// pub:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::common::*;
    use crate::fitness;
    use crate::operators::selection::ElitistSelection;
    use crate::operators::variation::OnePointCrossOver;

    #[test]
    fn test_engine() -> Result<()> {
        let population = build_initial_population(10);

        // create a breeder for new individuals
        let mut breeder = crate::gears::breeder::GeneticBreeder::new()
            .with_crossover(OnePointCrossOver)
            .with_selector(ElitistSelection(2));

        let mut engine = Engine::new(population)
            .with_fitness(fitness::Maximize)
            .with_creator(OneMax)
            .with_breeder(breeder);

        for g in engine.evolve().take(10) {
            let generation = g?;
            generation.summary();
        }

        Ok(())
    }
}
// test:1 ends here
