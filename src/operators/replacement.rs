// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use super::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
#[derive(Debug)]
/// Replace all bad performaing individuals in population with feasible
/// candidates.
pub struct FullReplacement;

impl ReplacementOperator for FullReplacement {
    fn remove_from<G: Genome, R: Rng + Sized>(
        &self,
        n: usize,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        unimplemented!()
    }
}
// base:1 ends here
