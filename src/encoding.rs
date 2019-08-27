// imports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*imports][imports:1]]
use std::fmt::Display;
use std::iter::FromIterator;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
#[derive(Clone, Debug, PartialEq)]
pub struct Binary(Vec<bool>);

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

impl Binary {
    /// Convert from binary string representation, e.g. "110"
    pub fn from_str(s: &str) -> Self {
        Self::from_iter(s.chars().map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!("bad char: {}", c),
        }))
    }
}
// base:1 ends here

// test

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*test][test:1]]
#[test]
fn test_binary() {
    let x = Binary(vec![true, false, true]);
    let s = format!("{}", x);
    let y = Binary::from_str(&s);
    assert_eq!(x, y);
}
// test:1 ends here
