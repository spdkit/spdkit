// [[file:../spdkit.note::*imports][imports:1]]
use super::*;

use gchemol::Molecule;
use vecfx::*;

type Point3 = [f64; 3];
type Array3x3 = [Point3; 3];
// imports:1 ends here

// [[file:../spdkit.note::a89cd4b4][a89cd4b4]]
fn get_moment_of_inertia(mol: &Molecule) -> ([f64; 3], [[f64; 3]; 3]) {
    let (evalues, vectors) = get_eigen_values_and_vectors(mol.inertia_matrix());

    // sort the eigenvalues in ascending order
    let indices: Vec<_> = evalues
        .iter()
        .enumerate()
        .sorted_by_key(|x| OrderedFloat(*x.1))
        .map(|x| x.0)
        .collect();

    // sort the corresponding eigenvectors in ascending order
    let mut evalues_ = evalues;
    let mut vectors_ = vectors;
    for (k, &i) in indices.iter().enumerate() {
        evalues_[k] = evalues[i];
        vectors_[k] = vectors[i];
    }

    (evalues_, vectors_)
}

fn get_matrix_trace(mat: &Array3x3) -> f64 {
    (0..3).map(|i| mat[i][i]).sum()
}

/// Calculate similarity using proposed by Lazauskas et al (DOI:10.1039/C6NR09072A)
pub(self) fn get_disparity_between(mol1: &Molecule, mol2: &Molecule) -> f64 {
    let mat1 = mol1.inertia_matrix();
    let mat2 = mol2.inertia_matrix();
    let trace1 = get_matrix_trace(&mat1);
    let trace2 = get_matrix_trace(&mat2);

    let evalues1 = get_eigen_values(mat1);
    let evalues2 = get_eigen_values(mat2);

    (0..3)
        .map(|i| {
            let lambda1 = evalues1[i];
            let lambda2 = evalues2[i];
            (lambda1 / trace1 - lambda2 / trace2).abs()
        })
        .sum()
}
// a89cd4b4 ends here

// [[file:../spdkit.note::*impl/nalgebra][impl/nalgebra:1]]
fn get_eigen_values_and_vectors(mat3x3: [[f64; 3]; 3]) -> ([f64; 3], [[f64; 3]; 3]) {
    let mat: Matrix3f = mat3x3.into();
    let eigen = mat.symmetric_eigen();
    let mut evalues = [0f64; 3];
    let mut vectors = [evalues; 3];

    for i in 0..3 {
        evalues[i] = eigen.eigenvalues[i];
        let vi = eigen.eigenvectors.column(i);
        vectors[i] = [vi[0], vi[1], vi[2]];
    }

    (evalues, vectors)
}

fn get_eigen_values(mat3x3: Array3x3) -> Point3 {
    let mat: Matrix3f = mat3x3.into();
    let eigen = mat.symmetric_eigen();
    let mut evalues = eigen.eigenvalues.as_slice().to_vec();
    evalues.sort_by_float();
    [evalues[0], evalues[1], evalues[2]]
}
// impl/nalgebra:1 ends here

// [[file:../spdkit.note::a281dbc9][a281dbc9]]
pub trait SimilarityExt {
    fn disparity_between(&self, mol: &Molecule) -> f64;
}

impl SimilarityExt for Molecule {
    /// Calculate disparity between `self` and `mol` using algorithm
    /// proposed by Lazauskas et al (DOI:10.1039/C6NR09072A)
    fn disparity_between(&self, mol: &Molecule) -> f64 {
        get_disparity_between(self, mol)
    }
}
// a281dbc9 ends here

// [[file:../spdkit.note::928243c2][928243c2]]
#[test]
fn test_principle_axes() -> Result<()> {
    use gchemol::prelude::*;
    use gchemol::Molecule;
    use vecfx::*;

    let mol = Molecule::from_file("./tests/files/H2O.xyz")?;
    let (eigen_values, _eigen_vectors) = get_moment_of_inertia(&mol);
    let eigen_values_expected = [0.54964496, 1.23918745, 1.78883242];
    approx::assert_relative_eq!(eigen_values.to_vector(), eigen_values_expected.to_vector(), epsilon = 1e-4);
    // NOTE: eigen vectors are not unique, and not easy to test

    let mol1 = Molecule::from_file("./tests/files/H2O.xyz")?;
    let mol2 = Molecule::from_file("./tests/files/CH4.xyz")?;
    let x1 = get_disparity_between(&mol1, &mol2);

    let mol2 = Molecule::from_file("./tests/files/H2O-rotated.mol2")?;
    let mol3 = Molecule::from_file("./tests/files/H2O-reordered.mol2")?;
    let x2 = get_disparity_between(&mol1, &mol2);
    assert!(x2 < 1e-3, "disparity is too large: {}", x2);
    let x3 = get_disparity_between(&mol1, &mol3);
    assert!(x3 < 1e-3, "disparity is too large: {}", x3);

    assert!(x1 > x2);

    Ok(())
}
// 928243c2 ends here
