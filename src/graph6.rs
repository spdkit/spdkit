// [[file:../spdkit.note::*imports][imports:1]]
use super::*;
// imports:1 ends here

// [[file:../spdkit.note::*R(x)][R(x):1]]
/// represent bit vector as bytes.
fn graph6_repr_bits(x: &[bool]) -> Vec<u8> {
    x.chunks(6)
        .map(|x| {
            if x.len() == 6 {
                // Split into groups of 6 bits each
                to_u8(x.to_vec()) + 63
            } else {
                // "Pad on the right with 0 to make the length a multiple of 6"
                let x: Vec<_> = x.iter().cloned().chain(std::iter::repeat(false)).take(6).collect();
                to_u8(x) + 63
            }
        })
        .collect()
}

#[test]
fn test_graph6_repr() {
    let bits = "1000101100011100";
    let bits: Vec<_> = bits.chars().map(|b| if b == '1' { true } else { false }).collect();
    let s = graph6_repr_bits(&bits);
    assert_eq!(s[0], 97);
    assert_eq!(s[1], 112);
    assert_eq!(s[2], 111);
}
// R(x):1 ends here

// [[file:../spdkit.note::*nodes][nodes:1]]
/// Encode a small nonnegative integers to graph6 bytes
fn encode_nodes(n: usize) -> Vec<u8> {
    match n {
        // a single byte
        0..=62 => vec![(n + 63) as u8],
        63..=258047 => {
            // 18-bit binary form of n
            let bits = format!("{:018b}", n);
            let bits: Vec<_> = bits.chars().map(|x| if x == '0' { false } else { true }).collect();
            let mut bytes = vec![126];
            bytes.extend(graph6_repr_bits(&bits));
            bytes
        }
        // eight bytes: 2 + 6
        258048..=68719476735 => {
            // 36-bit binary form of n
            let bits = format!("{:036b}", n);
            let bits: Vec<_> = bits.chars().map(|x| if x == '0' { false } else { true }).collect();
            let mut bytes = vec![126; 2];
            bytes.extend(graph6_repr_bits(&bits));
            bytes
        }
        _ => panic!("n is out of range: {:?}", n),
    }
}

#[test]
fn test_graph6_encode_node_n() {
    let x = encode_nodes(5);
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 68);

    let values = vec![126, 66, 63, 120];
    let encoded = encode_nodes(12345);
    assert_eq!(values.len(), encoded.len());
    for (a, b) in values.into_iter().zip(encoded.into_iter()) {
        assert_eq!(a, b);
    }

    let values = vec![126, 126, 63, 90, 90, 90, 90, 90];
    let encoded = encode_nodes(460175067);
    assert_eq!(values.len(), encoded.len());
    for (a, b) in values.into_iter().zip(encoded.into_iter()) {
        assert_eq!(a, b);
    }
}
// nodes:1 ends here

// [[file:../spdkit.note::*edges][edges:1]]
use gchemol::Molecule;

// the upper triangle of the adjacency matrix of molecule graph
fn encode_edeges(mol: &Molecule) -> Vec<u8> {
    let n = mol.natoms();
    let m = ((n * (n - 1)) as f64 / 2.0) as usize;

    let numbers: Vec<_> = mol.numbers().collect();
    let mut bits = vec![false; m];
    // (i,j),(i,j),(i,j),(i,j),(i,j),(i,j),...
    // (0,1),(0,2),(1,2),(0,3),(1,3),(2,3),...,(n-1,n).
    //   1     2     3     4     5     6   ...
    for j in 1..n {
        for i in 0..j {
            let ni = numbers[i];
            let nj = numbers[j];
            if mol.get_bond(ni, nj).is_some() {
                let k = upper_triangle_index(i, j);
                bits[k] = true;
            }
        }
    }
    graph6_repr_bits(&bits)
}

/// get index of i,j pairs using the ordering
/// (0,1),(0,2),(1,2),(0,3),(1,3),(2,3),...,(n-1,n).
fn upper_triangle_index(i: usize, j: usize) -> usize {
    assert!(j > i, "invalid index: {:?}", (i, j));
    ((j * (j - 1)) as f64 / 2.0) as usize + i
}

// convert 6 bits into a u8 byte
fn to_u8(slice: Vec<bool>) -> u8 {
    assert_eq!(slice.len(), 6);
    slice.iter().map(|&b| if b { 1 } else { 0 }).fold(0, |acc, b| (acc * 2 + b)) as u8
}

#[test]
fn test_to_u8() {
    let x: Vec<_> = "110000".chars().map(|x| if x == '1' { true } else { false }).collect();
    assert_eq!(to_u8(x), 48);
    let x: Vec<_> = "100010".chars().map(|x| if x == '1' { true } else { false }).collect();
    assert_eq!(to_u8(x), 34);
}

#[test]
fn test_graph6_encode_edges() -> Result<()> {
    use gchemol::prelude::*;

    let mol = Molecule::from_file("./tests/files/CH4-nauty.mol2")?;
    let bytes = encode_edeges(&mol);
    assert_eq!(bytes.len(), 2);
    assert_eq!(bytes[0], 81);
    assert_eq!(bytes[1], 99);

    Ok(())
}
// edges:1 ends here

// [[file:../spdkit.note::cccb03f9][cccb03f9]]
/// Encode molecule as string in graph6 format.
pub fn encode_molecule_as_graph6(mol: &Molecule) -> String {
    let bytes_nodes = encode_nodes(mol.natoms());
    let bytes_edges = encode_edeges(mol);
    bytes_nodes.into_iter().chain(bytes_edges).map(|x| x as char).collect()
}
// cccb03f9 ends here

// [[file:../spdkit.note::651a1baa][651a1baa]]
#[test]
fn test_encode_molecule_as_graph6() -> Result<()> {
    use gchemol::prelude::*;

    let mol = Molecule::from_file("./tests/files/CH4-nauty.mol2")?;
    let s = encode_molecule_as_graph6(&mol);
    assert_eq!(s, "DQc");

    Ok(())
}
// 651a1baa ends here
