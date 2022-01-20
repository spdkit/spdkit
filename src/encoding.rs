// [[file:../spdkit.note::*imports][imports:1]]
use std::fmt::Display;
use std::iter::FromIterator;

use crate::random::*;
// imports:1 ends here

// [[file:../spdkit.note::e2e7a684][e2e7a684]]
#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Binary(Vec<bool>);

impl crate::individual::Genome for Binary {}

// Print as binary string, e.g. 110101.
impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let v: String = self.0.iter().map(|&b| if b { '1' } else { '0' }).collect();
        write!(f, "{:}", v)
    }
}

impl FromIterator<bool> for Binary {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        Self {
            0: iter.into_iter().collect(),
        }
    }
}

impl std::ops::Deref for Binary {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Binary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Binary {
    /// Construct from a list of bool.
    pub fn new(list: Vec<bool>) -> Self {
        Self(list)
    }

    /// Convert from binary string representation, e.g. "110"
    pub fn from_str(s: &str) -> Self {
        Self::from_iter(s.chars().map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!("bad char: {}", c),
        }))
    }

    /// Flip the bits in specified `positions`.
    pub fn flip(&mut self, positions: impl IntoIterator<Item = usize>) {
        for i in positions.into_iter() {
            self.0[i] = !self.0[i];
        }
    }
}
// e2e7a684 ends here

// [[file:../spdkit.note::9defdebe][9defdebe]]
// impl crate::individual::Genome for gchemol::Molecule {}
// 9defdebe ends here

// [[file:../spdkit.note::*mutate][mutate:1]]
pub trait Mutate {
    /// Mutate `n` bits randomly.
    fn mutate<R: Rng + Sized>(&mut self, n: usize, rng: &mut R);
}

impl Mutate for Binary {
    /// Mutate `n` bits randomly.
    fn mutate<R: Rng + Sized>(&mut self, n: usize, rng: &mut R) {
        let mut choices: Vec<_> = (0..self.len()).collect();
        let positions = choices.choose_multiple(rng, n).cloned();
        self.flip(positions);
    }
}
// mutate:1 ends here

// [[file:../spdkit.note::*test][test:1]]
#[test]
fn test_binary() {
    let x = Binary(vec![true, false, true]);
    let s = x.to_string();
    assert_eq!(s, "101");
    let y = Binary::from_str(&s);
    assert_eq!(x, y);

    let mut y = y.clone();
    y.flip(vec![0]);

    assert_eq!(y.to_string(), "001");
}
// test:1 ends here
