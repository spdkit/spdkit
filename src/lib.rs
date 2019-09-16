// header

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*header][header:1]]
//===============================================================================#
//   DESCRIPTION:  spdkit: Structure Predication Development Kit
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  rewrite my python codes using rust
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 2 or upper
//       CREATED:  <2018-06-14 Thu 20:52>
//       UPDATED:  <2019-09-16 Mon 11:46>
//===============================================================================#
// header:1 ends here

// base

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*base][base:1]]
#[macro_use]
extern crate lazy_static;

use gchemol::prelude::*;
use gchemol::*;

pub(crate) mod common {
    pub use quicli::prelude::*;
    pub type Result<T> = ::std::result::Result<T, Error>;

    // Arbitrarily decide the order of NaNs
    macro_rules! local_float_cmp {
        ($fi:ident, $fj:ident) => {
            match ($fi.is_nan(), $fj.is_nan()) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                (true, true) => std::cmp::Ordering::Equal,
                (false, false) => unreachable!(),
            }
        };
    }

    // https://stackoverflow.com/questions/43921436/extend-iterator-with-a-mean-method
    pub trait FloatIteratorExt {
        fn fmax(mut self) -> Option<f64>;
        fn fmin(mut self) -> Option<f64>;
        fn imax(mut self) -> Option<(usize, f64)>;
        fn imin(mut self) -> Option<(usize, f64)>;
    }

    impl<F, T> FloatIteratorExt for T
    where
        T: Iterator<Item = F>,
        F: std::borrow::Borrow<f64>,
    {
        /// Returns the minimum element of an iterator. Return None if the
        /// iterator is empty.
        fn fmax(mut self) -> Option<f64> {
            if let Some(value) = self.next() {
                let f = self.fold(*value.borrow(), |a, b| a.max(*b.borrow()));
                Some(f)
            } else {
                None
            }
        }

        fn fmin(mut self) -> Option<f64> {
            if let Some(value) = self.next() {
                let f = self.fold(*value.borrow(), |a, b| a.min(*b.borrow()));
                Some(f)
            } else {
                None
            }
        }

        /// Find maximum value and the corresponding index. Return None if the
        /// iterator is empty.
        fn imax(mut self) -> Option<(usize, f64)> {
            if let Some(value) = self.next() {
                let value = *value.borrow();
                let value = (1..).zip(self).fold((0, value), |a, b| {
                    let (ia, fa) = a;
                    let (ib, fb) = b;
                    let fb = *fb.borrow();

                    if fb > fa {
                        (ib, fb)
                    } else {
                        (ia, fa)
                    }
                });
                Some(value)
            } else {
                None
            }
        }

        /// Find minimum value and the corresponding index. Return None if the
        /// iterator is empty.
        fn imin(mut self) -> Option<(usize, f64)> {
            if let Some(value) = self.next() {
                let value = *value.borrow();
                let value = (1..).zip(self).fold((0, value), |a, b| {
                    let (ia, fa) = a;
                    let (ib, fb) = b;
                    let fb = *fb.borrow();

                    if fb < fa {
                        (ib, fb)
                    } else {
                        (ia, fa)
                    }
                });
                Some(value)
            } else {
                None
            }
        }
    }

    /// For sort values in maximum first order.
    pub fn float_ordering_maximize(fi: &f64, fj: &f64) -> std::cmp::Ordering {
        fj.partial_cmp(&fi)
            .unwrap_or_else(|| local_float_cmp!(fi, fj))
    }

    /// For sort values in minimum first order.
    pub fn float_ordering_minimize(fi: &f64, fj: &f64) -> std::cmp::Ordering {
        fi.partial_cmp(&fj)
            .unwrap_or_else(|| local_float_cmp!(fi, fj))
    }

    #[test]
    fn test_float_ordering() {
        let mut values = vec![1.0, -1.0, std::f64::NAN, 0.5, 2.0];
        let m = values.iter().fmax();
        assert_eq!(m, Some(2.0));

        let m = values.iter().fmin();
        assert_eq!(m, Some(-1.0));

        let m = values.iter().imax();
        assert_eq!(m, Some((4, 2.0)));

        let m = values.iter().imin();
        assert_eq!(m, Some((1, -1.0)));

        values.sort_by(|a, b| float_ordering_maximize(&a, &b));
        assert_eq!(values[0], 2.0);
        assert!(values[4].is_nan());

        values.sort_by(|a, b| float_ordering_minimize(&a, &b));
        assert_eq!(values[0], -1.0);
        assert!(values[4].is_nan());
    }
}
// base:1 ends here

// mods

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*mods][mods:1]]
#[macro_use]
pub mod random; // the mod order is important for get_rng! macro

pub mod encoding;
pub mod engine;
pub mod fitness;
pub mod gears;
pub mod individual;
pub mod operators;
pub mod population;
pub mod termination;
mod annealing;
// mods:1 ends here

// prelude
// exports traits

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*prelude][prelude:1]]
pub mod prelude {
    pub use crate::individual::Create;
    pub use crate::individual::EvaluateObjectiveValue;
    pub use crate::random::*;
    pub use crate::operators::*;

    pub use crate::encoding::Mutate;
}
// prelude:1 ends here

// exports

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*exports][exports:1]]
pub use crate::engine::Engine;
pub use crate::gears::GeneticBreeder;
pub use crate::gears::Valuer;
pub use crate::individual::{Genome, Individual};
pub use crate::population::Population;
// exports:1 ends here

// src

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::*src][src:1]]
use crate::common::*;
use std::collections::HashSet;

use gchemol::{geometry::rand_rotate, io};

// return indices of atoms lying above the cutting plane
fn indices_above_plane(positions: &Vec<[f64; 3]>) -> HashSet<usize> {
    positions
        .iter()
        .enumerate()
        .filter(|(i, p)| p[2].is_sign_positive())
        .map(|(i, p)| i)
        .collect()
}

// cut the molecule into two parts using a random plane
fn cut_molecule_by_rand_plane(mol: &Molecule) -> (HashSet<usize>, HashSet<usize>, Vec<[f64; 3]>) {
    let natoms = mol.natoms();

    let mut mol = mol.clone();
    mol.recenter();

    let positions = mol.positions();
    let rotated = rand_rotate(&positions);

    let ind_all: HashSet<_> = (0..natoms).collect();
    let ind_above = indices_above_plane(&rotated);
    let ind_below = ind_all.difference(&ind_above).map(|x| *x).collect();

    (ind_above, ind_below, rotated)
}

#[deprecated(since = "0.0.2", note = "Will be removed soon")]
pub fn plane_cut_and_splice(mol1: &Molecule, mol2: &Molecule) -> Result<Molecule> {
    let natoms = mol1.natoms();
    // sanity check
    if mol2.natoms() == natoms {
        for (a1, a2) in mol1.atoms().zip(mol2.atoms()) {
            if a1.symbol() != a2.symbol() {
                bail!("atom numbering is inconsistent!");
            }
        }
    } else {
        bail!("molecules in difference size.");
    }

    // record element symbols
    let symbols = mol1.symbols();
    let reduced_symbols = mol1.reduced_symbols();
    let maxloop = 50000;
    let mut iloop = 0;

    let mut omol = mol1.clone();
    loop {
        let (above1, below1, rotated1) = cut_molecule_by_rand_plane(&mol1);
        let (above2, below2, rotated2) = cut_molecule_by_rand_plane(&mol2);
        debug!("above1 {:?}", above1);
        debug!("below2 {:?}", below2);
        // check if number of atoms is correct
        if above1.len() + below2.len() == natoms {
            // check if element types is correct

            let s1: Vec<_> = above1.iter().map(|&i| symbols[i]).collect();

            let mut s2: Vec<_> = below2.iter().map(|&i| symbols[i]).collect();

            s2.extend(s1.iter());

            assert_eq!(natoms, s2.len());
            omol.set_symbols(&s2);
            let mut got = true;
            for (k, v) in omol.reduced_symbols() {
                if reduced_symbols[&k] != v {
                    got = false;
                }
            }
            if got {
                // update positions
                let s1: Vec<_> = above1.iter().map(|&i| rotated1[i]).collect();

                let mut s2: Vec<_> = below2.iter().map(|&i| rotated2[i]).collect();

                s2.extend(s1.iter());

                omol.set_positions(&s2);
                break;
            }
        }

        iloop += 1;
        if iloop >= maxloop {
            bail!("max iterations reached.");
        }
    }

    Ok(omol)
}

#[test]
fn test_plane_cut_splice() {
    let mut mols1 = io::read("tests/files/c6h6-geom1.mol2").expect("geom1");
    let mut mol1 = &mut mols1[0];
    let mut mols2 = io::read("tests/files/c6h6-geom2.mol2").expect("geom2");
    let mut mol2 = &mut mols2[0];
    assert_eq!(mol1.natoms(), mol2.natoms());

    let x = plane_cut_and_splice(&mol1, &mol2).expect("plane-cut-and-splice");
    x.to_file("/tmp/aa.xyz").expect("write splice");
}
// src:1 ends here
