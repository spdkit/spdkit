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

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
use std::marker::PhantomData;

/// Evolution engine.
pub struct Engine<G, C, F, B>
where
    G: Genome,
    C: EvaluateObjectiveValue<G>,
    B: Breed<G>,
    F: EvaluateFitness<G>,
{
    mut_prob: f64,

    population: Option<Population<G>>,
    breeder: Option<B>,
    valuer: Option<Valuer<G, F, C>>,

    nlast: usize,
}
// base:1 ends here

// core

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*core][core:1]]
pub(crate) fn evolve_one_step<G, C, B, F, R>(
    cur_population: &Population<G>,
    breeder: &mut B,
    valuer: &mut Valuer<G, F, C>,
    rng: &mut R,
) -> Population<G>
where
    G: Genome,
    C: EvaluateObjectiveValue<G>,
    B: Breed<G>,
    F: EvaluateFitness<G>,
    R: Rng + Sized,
{
    // 1. create m new individuals from parent population.
    let m = cur_population.size();
    // 1.1 breed new genomes
    let new_genomes = breeder.breed(m, cur_population, rng);
    // 1.2 create new individuals from genomes.
    let mut new_indvs = valuer.create_individuals(new_genomes);
    println!("bred {} new individuals", new_indvs.len());

    // 2. create new population by supplanting bad performing individuals
    // 2.1 combine all available individuals into one.
    let old_indvs = cur_population.individuals();
    new_indvs.extend_from_slice(old_indvs);

    // 2.2 create a new population from combined new individuals
    let nlimit = cur_population.size_limit();
    let mut new_population = valuer.build_population(new_indvs).with_size_limit(nlimit);

    // 2.3 remove `n` low quality individuals
    let n = new_population.survive();
    println!("removed {} bad individuals.", n);
    new_population.to_owned()
}
// core:1 ends here

// pub

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*pub][pub:1]]
impl<G, C, F, B> Engine<G, C, F, B>
where
    G: Genome,
    C: EvaluateObjectiveValue<G>,
    B: Breed<G>,
    F: EvaluateFitness<G>,
{
    pub fn new() -> Self {
        Self {
            mut_prob: 0.1,

            breeder: None,
            population: None,
            valuer: None,

            nlast: 15,
        }
    }

    pub fn with_valuer(mut self, valuer: Valuer<G, F, C>) -> Self {
        self.valuer = Some(valuer);
        self
    }

    pub fn with_breeder(mut self, b: B) -> Self {
        self.breeder = Some(b);
        self
    }

    /// Evolves one step forward from seeds.
    ///
    /// # Parameters
    ///
    /// * seeds: genomes as initial seeds for evolution. The length of seeds
    /// will be set as the internal population size limit.
    ///
    /// # Returns
    ///
    /// * return an iterator over `Generation`.
    ///
    pub fn evolve<'a>(
        &'a mut self,
        seeds: &[G],
    ) -> impl Iterator<Item = Result<Generation<G>>> + 'a {
        let mut rng = get_rng!();
        let mut ig = 0;

        let mut termination = RunningMean::new(self.nlast);
        let mut breeder = self.breeder.take().expect("no breeder");
        let mut valuer = self.valuer.take().expect("no valuer");

        // create individuals, build population.
        let indvs = valuer.create_individuals(seeds.to_vec());
        let nlimit = seeds.len();
        let mut population = valuer.build_population(indvs).with_size_limit(nlimit);

        // enter main loop
        std::iter::from_fn(move || {
            if ig == 0 {
                println!("initial population:");
                for m in population.members() {
                    // println!("{}", m);
                }
            } else {
                let mut new_population =
                    evolve_one_step(&population, &mut breeder, &mut valuer, &mut *rng);

                population = new_population;
            }

            let g = Generation {
                index: ig,
                population: population.clone(),
            };
            ig += 1;

            // avoid infinite loop using a reliable termination criterion.
            if termination.meets(&g) {
                error!("Terminated for stagnation!");
                error!(
                    "Simulation has evolved for {} generations without changes.",
                    self.nlast
                );

                None
            } else {
                Some(Ok(g))
            }
        })
    }

    // /// Run the simulation until termination conditions met.
    // ///
    // /// # Returns
    // ///
    // /// * return the final `Generation`.
    // ///
    // pub fn run_until<T: Terminate>(
    //     &mut self,
    //     genomes: &[G],
    //     conditions: impl IntoIterator<Item = T>,
    // ) -> Result<Generation<G>> {
    //     let mut conditions: Vec<_> = conditions.into_iter().collect();
    //     for g in self.evolve(genomes) {
    //         let generation = g?;
    //         for t in conditions.iter_mut() {
    //             if t.meets(&generation) {
    //                 return Ok(generation);
    //             }
    //         }
    //     }
    //     unreachable!()
    // }
}
// pub:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]
#[cfg(test)]
mod test {
    use super::*;
    use crate::common::*;
    use crate::fitness;
    use crate::operators::selection::RouletteWheelSelection;
    use crate::operators::variation::OnePointCrossOver;

    #[test]
    fn test_engine() -> Result<()> {
        // create a breeder gear
        let breeder = crate::gears::GeneticBreeder::new()
            .with_crossover(OnePointCrossOver)
            .with_selector(RouletteWheelSelection::new(2));

        // create a valuer gear
        let valuer = Valuer::new()
            .with_fitness(fitness::Maximize)
            .with_creator(OneMax);

        let mut engine = Engine::new().with_valuer(valuer).with_breeder(breeder);

        let seeds = build_initial_genomes(10);
        for g in engine.evolve(&seeds).take(10) {
            let generation = g?;
            generation.summary();
        }

        Ok(())
    }

    // test only
    fn build_initial_genomes(n: usize) -> Vec<Binary> {
        // generate `n` binary genomes in size of 10.
        (0..n).map(|_| random_binary(11)).collect()
    }

    fn random_binary(length: usize) -> Binary {
        let mut rng = get_rng!();
        let list: Vec<_> = (0..length).map(|_| rng.gen()).collect();
        Binary::new(list)
    }
}
// test:1 ends here
