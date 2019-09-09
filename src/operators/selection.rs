// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::individual::*;
use crate::population::*;
use crate::random::*;

use super::*;
// imports:1 ends here

// random selection

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*random%20selection][random selection:1]]
/// Select individuals from population at random.
#[derive(Debug)]
pub struct RandomSelection {
    n: usize,
    allow_repetition: bool,
}

impl RandomSelection {
    /// Select individuals randomly from `population`.
    fn select<'a, G, R>(&self, population: &'a Population<G>, rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized,
    {
        let all_members: Vec<_> = population.members().collect();
        if self.allow_repetition {
            let mut selected = vec![];
            for _ in 0..self.n {
                let member = all_members
                    .choose(rng)
                    .expect("cannot select from empty slice");
                selected.push(member.clone());
            }

            selected
        } else {
            all_members.choose_multiple(rng, self.n).cloned().collect()
        }
    }
}

impl<'a> SelectionOperator<'a> for RandomSelection {
    /// Select individuals randomly from `population`.
    fn select_from<G, R>(
        &self,
        population: &'a Population<G>,
        rng: &mut R,
    ) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized,
    {
        self.select(population, rng)
    }
}
// random selection:1 ends here

// elitist selection

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*elitist%20selection][elitist selection:1]]
/// ElitistSelection is a simple selection strategy where a limited number of
/// individuals with the best fitness values are chosen.
#[derive(Debug)]
pub struct ElitistSelection(pub usize);

impl ElitistSelection {
    /// Select `n` best members from `population`.
    fn select<'a, G>(&self, population: &'a Population<G>) -> Vec<Member<'a, G>>
    where
        G: Genome,
    {
        // Reversely sort members by fitnesses.
        let mut members: Vec<_> = population.members().collect();
        members.sort_by_fitness();
        members[..self.0].to_vec()
    }
}

impl<'a> SelectionOperator<'a> for ElitistSelection {
    fn select_from<G, R>(&self, population: &'a Population<G>, _rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized,
    {
        self.select(population)
    }
}
// elitist selection:1 ends here

// roulette wheel

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*roulette%20wheel][roulette wheel:1]]
/// Fitness proportionate selection.
///
/// # Reference
///
/// https://en.wikipedia.org/wiki/Fitness_proportionate_selection
///
/// # Panic
///
/// * panic if individual fitness is negative.
///
#[derive(Debug)]
pub struct RouletteWheelSelection {
    // Select `n` individuals
    n: usize,
    // Allow selected individuals repeat or not
    allow_repetition: bool,
}

impl RouletteWheelSelection {
    fn select<'a, G, R>(&self, population: &'a Population<G>, rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized,
    {
        let mut choices: Vec<_> = population.members().enumerate().collect();
        let mut selected: Vec<_> = vec![];

        if self.allow_repetition {
            for _ in 0..self.n {
                let (_, m) = choices
                    .choose_weighted(rng, |(_, m)| m.fitness)
                    .unwrap_or_else(|e| panic!("Weighted selection failed: {:?}", e));
                selected.push(m.clone());
            }
        } else {
            for _ in 0..self.n {
                // avoid: mutable_borrow_reservation_conflict
                // https://github.com/rust-lang/rust/issues/59159
                let i = {
                    let (i, m) = choices
                        .choose_weighted(rng, |(_, m)| m.fitness)
                        .unwrap_or_else(|e| panic!("Weighted selection failed: {:?}", e));
                    selected.push(m.clone());
                    *i
                };
                choices.remove(i);
            }
        }

        selected
    }
}

impl<'a> SelectionOperator<'a> for RouletteWheelSelection {
    fn select_from<G, R>(&self, population: &'a Population<G>, rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized,
    {
        self.select(population, rng)
    }
}
// roulette wheel:1 ends here

// tournament selection

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*tournament%20selection][tournament selection:1]]
/// Divide the populations into multiple parts and select the best one from each
/// part in a deterministic way.
///
/// This implementation is a little bit different from the one described in the
/// wikipedia article
#[derive(Debug)]
pub struct TournamentSelection {
    n: usize,
}
impl TournamentSelection {
    fn select<'a, G, R>(&self, population: &'a Population<G>, rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized,
    {
        let psize = population.size();
        assert!(psize >= self.n, "select too many individuals!");

        let tsize = (psize as f64 / self.n as f64).floor() as usize;

        let mut members: Vec<_> = population.members().collect();
        members.shuffle(rng);

        members
            .chunks_mut(tsize)
            .map(|mut part| {
                // sort members by individual fitness
                part.sort_by_fitness();
                part[0].clone()
            })
            .collect()
    }
}

impl<'a> SelectionOperator<'a> for TournamentSelection {
    fn select_from<G, R>(&self, population: &'a Population<G>, rng: &mut R) -> Vec<Member<'a, G>>
    where
        G: Genome,
        R: Rng + Sized,
    {
        self.select(population, rng)
    }
}
// tournament selection:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]

// test:1 ends here
