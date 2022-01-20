// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::iter::FromIterator;

use crate::common::*;
use crate::encoding::*;
use crate::fitness::*;
use crate::gears::*;
use crate::individual::*;
use crate::operators::*;
use crate::population::*;
use crate::random::*;
use crate::termination::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
use std::marker::PhantomData;

pub trait Evolve<G, F, C>
where
    G: Genome,
    F: EvaluateFitness<G>,
    C: EvaluateObjectiveValue<G>,
{
    // Define how to evolve to next generation.
    fn next_generation(&mut self, cur_population: &Population<G>, valuer: &mut Valuer<G, F, C>) -> Population<G>;
}
// base:1 ends here

// core

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*core][core:1]]
pub struct EvolutionAlgorithm<G, B, S>
where
    G: Genome,
    B: Breed<G>,
    S: Survive<G>,
{
    breeder: B,
    survivor: S,
    _g: PhantomData<G>,
}

impl<G, B, S> EvolutionAlgorithm<G, B, S>
where
    G: Genome,
    B: Breed<G>,
    S: Survive<G>,
{
    pub fn new(breeder: B, survivor: S) -> Self {
        Self {
            breeder,
            survivor,
            _g: PhantomData,
        }
    }
}

impl<G, C, B, S, F> Evolve<G, F, C> for EvolutionAlgorithm<G, B, S>
where
    G: Genome,
    C: EvaluateObjectiveValue<G>,
    B: Breed<G>,
    S: Survive<G>,
    F: EvaluateFitness<G>,
{
    fn next_generation(&mut self, cur_population: &Population<G>, valuer: &mut Valuer<G, F, C>) -> Population<G> {
        let mut rng = get_rng!();
        evolve_one_step(cur_population, &mut self.breeder, &mut self.survivor, valuer, &mut *rng)
    }
}

fn evolve_one_step<G, C, B, S, F, R>(
    cur_population: &Population<G>,
    breeder: &mut B,
    survivor: &mut S,
    valuer: &mut Valuer<G, F, C>,
    rng: &mut R,
) -> Population<G>
where
    G: Genome,
    C: EvaluateObjectiveValue<G>,
    B: Breed<G>,
    S: Survive<G>,
    F: EvaluateFitness<G>,
    R: Rng + Sized,
{
    // 1. create new individuals from parent population.
    // 1.1 breed new genomes
    let new_genomes = breeder.breed(cur_population.size_limit(), cur_population, rng);
    // 1.2 create new individuals from genomes.
    let mut new_indvs = valuer.create_individuals(new_genomes);
    println!("bred {} new individuals", new_indvs.len());

    // 2. create new population by supplanting bad performing individuals
    // 2.1 combine all available individuals into one.
    let old_indvs = cur_population.individuals();
    new_indvs.extend_from_slice(old_indvs);

    // 2.2 create a new population from combined new individuals
    let nlimit = cur_population.size_limit();
    let tmp_population = valuer.build_population(new_indvs).with_size_limit(nlimit);
    let m = tmp_population.size();

    // 2.3 remove low quality individuals
    let survived_indvs = survivor.survive(tmp_population, rng);
    let n = m - survived_indvs.len();
    println!("removed {} bad individuals.", n);
    let mut new_population = valuer.build_population(survived_indvs).with_size_limit(nlimit);

    new_population.to_owned()
}
// core:1 ends here

// pub

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*pub][pub:1]]
/// Evolution engine.
pub struct Engine<G, E, F, C>
where
    G: Genome,
    E: Evolve<G, F, C>,
    F: EvaluateFitness<G>,
    C: EvaluateObjectiveValue<G>,
{
    nlast: usize,

    algo: Option<E>,
    valuer: Option<Valuer<G, F, C>>,
    population: Option<Population<G>>,
}

impl<G, E, F, C> Engine<G, E, F, C>
where
    G: Genome,
    E: Evolve<G, F, C>,
    F: EvaluateFitness<G>,
    C: EvaluateObjectiveValue<G>,
{
    pub fn create() -> Self {
        Self {
            nlast: 30,

            algo: None,
            valuer: None,
            population: None,
        }
    }

    /// Set core algorithm for evolution
    pub fn algorithm(mut self, algo: E) -> Self {
        self.algo = Some(algo);
        self
    }

    /// The last n generations for termination criterion.
    pub fn termination_nlast(mut self, n: usize) -> Self {
        self.set_termination_nlast(n);
        self
    }

    /// The last n generations for termination criterion.
    pub fn set_termination_nlast(&mut self, n: usize) {
        assert!(n > 1, "invalid nlast value");
        self.nlast = n;
    }

    /// Set Valuer for evolution.
    pub fn valuer(mut self, valuer: Valuer<G, F, C>) -> Self {
        self.valuer = Some(valuer);
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
    pub fn evolve<'a>(&'a mut self, seeds: &[G]) -> impl Iterator<Item = Result<Generation<G>>> + 'a {
        let mut termination = RunningMean::new(self.nlast);
        let mut valuer = self.valuer.take().expect("no valuer");
        let mut algo = self.algo.take().expect("no algo");

        // create individuals, build population.
        let indvs = valuer.create_individuals(seeds.to_vec());
        let nlimit = seeds.len();
        let mut population = valuer.build_population(indvs).with_size_limit(nlimit);

        // enter main loop
        let mut ig = 0;
        std::iter::from_fn(move || {
            if ig == 0 {
                println!("initial population:");
                for m in population.members() {
                    // println!("{}", m);
                }
            } else {
                let new_population = algo.next_generation(&population, &mut valuer);

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
                error!("Simulation has evolved for {} generations without changes.", self.nlast);

                None
            } else {
                Some(Ok(g))
            }
        })
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
    use crate::gears::*;
    use crate::operators::selection::RouletteWheelSelection;
    use crate::operators::variation::OnePointCrossOver;

    #[test]
    fn test_engine() -> Result<()> {
        // create a valuer gear
        let valuer = Valuer::new().with_fitness(fitness::Maximize).with_creator(OneMax);

        // create a survivor gear
        let survivor = Survivor::default();

        // create a breeder gear
        let breeder = crate::gears::GeneticBreeder::new()
            .with_crossover(OnePointCrossOver)
            .with_selector(RouletteWheelSelection::new(2));

        // setup the algorithm
        let algo = EvolutionAlgorithm::new(breeder, survivor);

        let seeds = build_initial_genomes(10);
        for g in Engine::create().valuer(valuer).algorithm(algo).evolve(&seeds).take(10) {
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
