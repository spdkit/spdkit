// [[file:../spdkit.note::97a15b9f][97a15b9f]]
use super::*;

use gchemol::prelude::*;
use gchemol::Molecule;
// 97a15b9f ends here

// [[file:../spdkit.note::*equivalent atoms][equivalent atoms:1]]
/// # Panics
///
/// * panics if number of atoms not matching
pub(crate) fn find_equivalent_atoms(mol1: &Molecule, mol2: &Molecule, reorder: bool) -> Option<Molecule> {
    use std::collections::HashMap;

    let mut mol2_old = mol2.clone();
    let mut mol1 = mol1.clone();
    let mut mol2 = mol2.clone();
    let numbers1 = mol1.numbers().collect_vec();
    let numbers2 = mol2.numbers().collect_vec();
    for (&i1, &i2) in numbers1.iter().zip(&numbers2) {
        let a1 = mol1.get_atom_unchecked_mut(i1);
        let a2 = mol2.get_atom_unchecked_mut(i2);
        a1.set_label(format!("{}", i1));
        a2.set_label(format!("{}", i2));
    }

    mol1.reorder_cannonically();
    mol2.reorder_cannonically();

    let m1: HashMap<usize, usize> = mol1.atoms().map(|(i, a)| (a.label().parse().unwrap(), i)).collect();
    let m2: HashMap<usize, usize> = mol2.atoms().map(|(i, a)| (i, a.label().parse().unwrap())).collect();

    for n in numbers1 {
        let new = &m1[&n];
        let n_equivalent = m2[&new];
        println!("{:^5} => {:^5}", n, n_equivalent);
        let a = mol2_old.get_atom_unchecked_mut(n_equivalent);
        a.properties.store("tag", n);
    }
    if reorder {
        let orders: Vec<usize> = mol2_old.atoms().map(|(_, a)| a.properties.load("tag").unwrap()).collect();
        mol2_old.reorder(&orders);
        mol2_old.into()
    } else {
        None
    }
}

#[test]
#[ignore]
fn test_molecule_reorder() -> Result<()> {
    let mut mol1 = Molecule::from_file("/tmp/1.mol2")?;
    let mut mol2 = Molecule::from_file("/tmp/2.mol2")?;

    let mol = find_equivalent_atoms(&mol1, &mol2, true).unwrap();
    mol.to_file("/tmp/22.mol2")?;

    Ok(())
}
// equivalent atoms:1 ends here

// [[file:../spdkit.note::63d99d8c][63d99d8c]]
/// Renumber `mol` based on connectivity of `mol_ref`
fn renumber_atoms_by_connectivity(mol: &mut Molecule, mol_ref: &Molecule) -> Result<f64> {
    use std::collections::HashMap;

    let mut mol_b = mol_ref.clone();
    ensure!(mol.fingerprint() == mol_b.fingerprint(), "found difference in connectivity");
    let (_, po) = mol_b.reorder_cannonically();

    // NOTE: useful for mirror inversion?
    let _ = mol.reorder_cannonically();
    let rmsd = mol.superimpose_onto(&mol_b, None);
    mol.renumber_using(&po);

    Ok(rmsd)
}
// 63d99d8c ends here

// [[file:../spdkit.note::edecb43c][edecb43c]]
fn reorder_atoms_canonically(mol: &mut Molecule) -> (Vec<usize>, Vec<usize>) {
    let nodes: Vec<_> = mol.numbers().collect();
    let edges: Vec<_> = mol.bonds().map(|(i, j, _)| (i, j)).collect();
    let colors: Vec<_> = mol.atomic_numbers().collect();

    let labels = nauty::get_canonical_labels(&nodes, &edges, &colors).expect("nauty failure");
    assert_eq!(labels.len(), nodes.len());
    // NOTE: make permutation into sorting order. it is tricky.
    let mapping: std::collections::HashMap<_, _> = labels.iter().enumerate().map(|(i, l)| (*l as usize, i + 1)).collect();
    let orders: Vec<_> = nodes.iter().map(|i| mapping[&i]).collect();
    mol.reorder(&orders);
    (orders, labels)
}
// edecb43c ends here

// [[file:../spdkit.note::3d0c2d80][3d0c2d80]]
/// Extension trait providing fingerprint method
pub trait FingerPrintExt {
    fn fingerprint(&self) -> String;
    fn reorder_cannonically(&mut self) -> (Vec<usize>, Vec<usize>);
    fn resemble_rigidly(&mut self, _: &Molecule) -> Result<f64>;
}

impl FingerPrintExt for Molecule {
    /// Return unique fingerprint of current molecule
    fn fingerprint(&self) -> String {
        let mut mol = self.clone();
        reorder_atoms_canonically(&mut mol);
        let fp = crate::graph6::encode_molecule_as_graph6(&mol);
        gut::utils::hash_code(&fp)
    }

    /// This is an operation of reordering the atoms in a way that
    /// does not depend on where they were before. The bonding graph
    /// is important for this operation. Return the orders and
    /// canonical labels applied. The latter can be applied to restore
    /// original numbering.
    ///
    /// # Example
    /// ```rust,ignore,no_run
    /// let (o1, o2) = mol.reorder_cannonically();
    /// # restore old numbering
    /// mol.reorder(&o2);
    /// ```
    fn reorder_cannonically(&mut self) -> (Vec<usize>, Vec<usize>) {
        reorder_atoms_canonically(self)
    }

    /// Make `self` resemble `mol_ref` by applying rigid operations in
    /// permutation, translation or rotation, without changing inner
    /// 3D geometry. Equivalent atoms are recoginized based on
    /// connectivity. Return alignment rmsd on success.
    fn resemble_rigidly(&mut self, mol_ref: &Molecule) -> Result<f64> {
        let rmsd = renumber_atoms_by_connectivity(self, mol_ref)?;
        Ok(rmsd)
    }
}
// 3d0c2d80 ends here

// [[file:../spdkit.note::fb3d7a90][fb3d7a90]]
#[test]
fn test_molecule_fingerprint() -> Result<()> {
    let mol1 = Molecule::from_file("./tests/files/H2O-rotated.mol2")?;
    let fp1 = mol1.fingerprint();

    let mol2 = Molecule::from_file("./tests/files/H2O-reordered.mol2")?;
    let fp2 = mol2.fingerprint();
    assert_eq!(fp1, fp2);

    let mut mol1_ = mol1.clone();
    let mut mol2_ = mol2.clone();
    let (new_numbers, _) = mol1_.reorder_cannonically();
    let _ = mol2_.reorder_cannonically();
    for i in 1..=3 {
        assert_eq!(mol1_.get_atom_unchecked(i).symbol(), mol2_.get_atom_unchecked(i).symbol());
    }
    let old_numbers = mol1.numbers();
    let mapping: std::collections::HashMap<usize, usize> = new_numbers.into_iter().zip(old_numbers).collect();
    for (i, j) in [(1, 2), (1, 3), (2, 3)] {
        let dij_old = mol1.distance(i, j);
        let dij_new = mol1_.distance(mapping[&i], mapping[&j]);
        assert_eq!(dij_old, dij_new);
    }

    let mol = Molecule::from_file("./tests/files/CH4-nauty.mol2")?;
    let fp3 = mol.fingerprint();
    assert_ne!(fp1, fp3);

    // restore old numbering
    let mut mol1_ = mol1.clone();
    let (o1, o2) = mol1_.reorder_cannonically();
    // NOTE: reorder will renumber atoms from 1
    // mol1_.reorder(&o2);
    mol1_.renumber_using(&o2);
    for i in 1..=3 {
        let ai = mol1.get_atom_unchecked(i);
        let bi = mol1_.get_atom_unchecked(i);
        assert_eq!(ai.symbol(), bi.symbol());
        assert_eq!(ai.position(), bi.position());
    }

    Ok(())
}

#[test]
fn test_reorder_by_connect() -> Result<()> {
    use gchemol::prelude::*;

    let mut ma = Molecule::from_file("tests/files/submol_a.mol2")?;
    let mb = Molecule::from_file("tests/files/submol_b.mol2")?;

    renumber_atoms_by_connectivity(&mut ma, &mb)?;
    for i in ma.numbers() {
        let aa = ma.get_atom_unchecked(i);
        let ab = mb.get_atom_unchecked(i);
        assert_eq!(aa.symbol(), ab.symbol());
    }

    Ok(())
}
// fb3d7a90 ends here
