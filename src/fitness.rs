// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use crate::common::*;
use crate::encoding::Binary;
use crate::individual::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
/// Evaluate the fitness of individual in population based on raw_score of
/// individual.
///
/// Fitness is a measure of quality of a solution (individual). A larger value
/// of fitness indicates a better individual in population.
///
/// Fitness evaluation should not be an expensive operation.
///
pub trait EvaluateFitness<G>
where
    G: Genome,
{
    fn evaluate(&self, indvs: &[Individual<G>]) -> Vec<f64>;
}

/// For Maximizing individual raw score. The larger of individual raw_score, the
/// larger of the fitness.
pub struct Maximize;

impl<G> EvaluateFitness<G> for Maximize
where
    G: Genome,
{
    fn evaluate(&self, indvs: &[Individual<G>]) -> Vec<f64> {
        if let Some(score_ref) = indvs.iter().map(|indv| indv.raw_score()).fmin() {
            indvs.iter().map(|x| x.raw_score() - score_ref).collect()
        } else {
            warn!("empty individual list!");
            vec![]
        }
    }
}

/// For minimizing individual raw score. The smaller of the raw_score, the
/// larger of the fitness.
pub struct Minimize;

impl<G> EvaluateFitness<G> for Minimize
where
    G: Genome,
{
    fn evaluate(&self, indvs: &[Individual<G>]) -> Vec<f64> {
        if let Some(score_ref) = indvs.iter().map(|indv| indv.raw_score()).fmax() {
            indvs.iter().map(|x| score_ref - x.raw_score()).collect()
        } else {
            warn!("empty individual list!");
            vec![]
        }
    }
}
// base:1 ends here

// minimize energy

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*minimize%20energy][minimize energy:1]]
/// The lower of the energy, the better of an individual
pub struct MinimizeEnergy {
    conversion: f64,
    temperature: f64,
}

impl MinimizeEnergy {
    pub fn new(temperature: f64) -> Self {
        assert!(
            temperature.is_sign_positive(),
            "temperature cannot be negative!"
        );

        Self {
            conversion: 96.0,
            temperature,
        }
    }

    pub fn unit(mut self, u: &str) -> Self {
        match u {
            "eV" => self.conversion = 96.0,
            "au" => self.conversion = 2625.5,
            "kcal" => self.conversion = 4.184,
            "kJ" => self.conversion = 1.0,
            _ => panic!("unkonw unit: {}", u),
        }
        self
    }
}

impl<G> EvaluateFitness<G> for MinimizeEnergy
where
    G: Genome,
{
    fn evaluate(&self, indvs: &[Individual<G>]) -> Vec<f64> {
        if let Some(score_ref) = indvs.iter().map(|indv| indv.raw_score()).fmax() {
            indvs
                .iter()
                .map(|x| {
                    let value = self.conversion * (score_ref - x.raw_score());
                    (value / (self.temperature * 0.0083145)).exp()
                })
                .collect()
        } else {
            warn!("empty individual list!");
            vec![]
        }
    }
}
// minimize energy:1 ends here
