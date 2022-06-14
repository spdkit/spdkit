// [[file:../spdkit.note::*header][header:1]]
//===============================================================================#
//   DESCRIPTION:  spdkit: Structure Predication Development Kit
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  rewrite my python codes using rust
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 2 or upper
//       CREATED:  <2018-06-14 Thu 20:52>
//       UPDATED:  <2022-06-14 Tue 10:56>
//===============================================================================#
// header:1 ends here

// [[file:../spdkit.note::3ac79127][3ac79127]]
use gut::prelude::*;

// FIXME: remove?

#[macro_use]
extern crate lazy_static;

pub mod common {
    pub use gut::prelude::*;

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
    pub trait FloatIteratorExt
    where
        Self: std::marker::Sized,
    {
        fn fmax(mut self) -> Option<f64> {
            todo!()
        }
        fn fmin(mut self) -> Option<f64> {
            todo!()
        }
        fn imax(mut self) -> Option<(usize, f64)> {
            todo!()
        }
        fn imin(mut self) -> Option<(usize, f64)> {
            todo!()
        }
    }

    impl<F, T> FloatIteratorExt for T
    where
        T: Iterator<Item = F>,
        F: std::borrow::Borrow<f64>,
        Self: std::marker::Sized,
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
        fj.partial_cmp(&fi).unwrap_or_else(|| local_float_cmp!(fi, fj))
    }

    /// For sort values in minimum first order.
    pub fn float_ordering_minimize(fi: &f64, fj: &f64) -> std::cmp::Ordering {
        fi.partial_cmp(&fj).unwrap_or_else(|| local_float_cmp!(fi, fj))
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
// 3ac79127 ends here

// [[file:../spdkit.note::1e9e2348][1e9e2348]]
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
mod fingerprint;
mod graph6;
mod vars;
// 1e9e2348 ends here

// [[file:../spdkit.note::4ccc7fd1][4ccc7fd1]]
pub mod prelude {
    pub use crate::engine::Evolve;
    pub use crate::fitness::EvaluateFitness;
    pub use crate::gears::Breed;
    pub use crate::gears::Survive;
    pub use crate::individual::EvaluateObjectiveValue;
    pub use crate::operators::*;
    pub use crate::population::SortMember;
    pub use crate::random::*;

    pub use crate::encoding::Mutate;
    pub use crate::fingerprint::FingerPrintExt;
}
// 4ccc7fd1 ends here

// [[file:../spdkit.note::*exports][exports:1]]
pub use crate::engine::{Engine, EvolutionAlgorithm};
pub use crate::gears::GeneticBreeder;
pub use crate::gears::Survivor;
pub use crate::gears::Valuer;
pub use crate::individual::{Genome, Individual};
pub use crate::population::Population;
// exports:1 ends here
