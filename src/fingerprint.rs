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

// [[file:../spdkit.note::*pub][pub:1]]
fn reorder_atoms_canonically(mol: &mut Molecule) {
    let nodes: Vec<_> = mol.numbers().collect();
    let edges: Vec<_> = mol.bonds().map(|(i, j, _)| (i, j)).collect();
    let colors: Vec<_> = mol.atomic_numbers().collect();

    let labels = nauty::get_canonical_labels(&nodes, &edges, &colors).expect("nauty failure");
    // NOTE: make permutation into sorting order. it is tricky.
    let mapping: std::collections::HashMap<_, _> = labels.into_iter().enumerate().map(|(i, l)| (l as usize, i)).collect();
    let orders: Vec<_> = nodes.iter().map(|i| mapping[&i]).collect();
    mol.reorder(&orders);
}

/// Extension trait providing fingerprint method
pub trait FingerPrintExt {
    fn fingerprint(&self) -> String;
    fn reorder_cannonically(&mut self);
}

impl FingerPrintExt for Molecule {
    /// Return unique fingerprint of current molecule
    fn fingerprint(&self) -> String {
        let mut mol = self.clone();
        reorder_atoms_canonically(&mut mol);
        crate::graph6::encode_molecule_as_graph6(&mol)
    }

    /// This is an operation of reordering the atoms in a way that does not depend
    /// on where they were before. The bonding graph is important for this
    /// operation.
    fn reorder_cannonically(&mut self) {
        reorder_atoms_canonically(self);
    }
}
// pub:1 ends here

// [[file:../spdkit.note::*test][test:1]]
#[test]
fn test_molecule_fingerprint() -> Result<()> {
    let mol = Molecule::from_file("./tests/files/H2O-rotated.mol2")?;
    let fp1 = mol.fingerprint();

    let mol = Molecule::from_file("./tests/files/H2O-reordered.mol2")?;
    let fp2 = mol.fingerprint();
    assert_eq!(fp1, fp2);

    let mol = Molecule::from_file("./tests/files/CH4-nauty.mol2")?;
    let fp3 = mol.fingerprint();
    assert_ne!(fp1, fp3);

    Ok(())
}
// test:1 ends here
