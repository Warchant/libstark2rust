mod glue;
mod sequence;

use glue::generate_valid_constraints;

use crate::{
    glue::libstark::{
        bair_witness_checker_verify_constraints_assignment,
        bair_witness_checker_verify_constraints_permutation,
    },
    sequence::Sequence,
};

fn main() {
    let (instance, witness) = generate_valid_constraints();
    assert!(bair_witness_checker_verify_constraints_assignment(
        instance.clone(),
        witness.clone()
    ));
    assert!(bair_witness_checker_verify_constraints_permutation(
        instance.clone(),
        witness.clone()
    ));

    let permutations = witness.get_permutations();
    println!(
        "Permutations: {:?} (size: {})",
        permutations.into_iter().collect::<Vec<_>>(),
        permutations.size()
    );

    let assignment = witness.get_assignment();
    println!(
        "Assignment: TODO (size: {})",
        permutations.size()
    );
}
