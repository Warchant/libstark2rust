mod glue;
mod sequence;

use std::ops::Deref;

use crate::{
    glue::libstark::{
        bair_witness_checker_verify_constraints_assignment,
        bair_witness_checker_verify_constraints_permutation,
        new_bair_witness,
        new_hardcoded_bair_instance,
        SharedColor,
    },
    sequence::Sequence,
};

fn main() {
    // NOTE(Bohdan): BairWitness::color_t is a std::vector of Algebra::FieldElement.
    // FieldElement has the only member - FFT::Element. Which, in turn, is
    // an array of cell_t (long long unsigned int == size_t). Array size is 1.
    // To simplify boundary C++/Rust we do not expose Algebra::FieldElement/FFT::Element.
    // Instead, we represent FFT::Element as size_t, so entire BairWitness::color_t
    // becomes Vec<usize>.

    // create assignment_t value
    let a: Vec<Vec<usize>> = vec![
        vec![1, 2, 3],
        vec![2, 3, 4],
        vec![9, 8, 7],
        vec![12, 3, 4],
        vec![0, 0, 7],
        vec![1, 1, 1],
        vec![2, 2, 2],
    ];

    // create permutation_t value
    let p: Vec<usize> = vec![0, 1, 2, 3, 4, 6, 5];

    let vector_len = 3;
    let domain_size_indicator = 3;
    // create BairWitness and BairInstance
    let witness = new_bair_witness(
        &a.clone()
            .into_iter()
            .map(|v| SharedColor { v })
            .collect::<Vec<_>>(),
        &p,
    );
    let instance = new_hardcoded_bair_instance(vector_len, domain_size_indicator);

    // verify constraints
    assert!(bair_witness_checker_verify_constraints_assignment(
        instance.clone(),
        witness.clone()
    ));
    assert!(bair_witness_checker_verify_constraints_permutation(
        instance.clone(),
        witness.clone()
    ));

    // print permutations
    let permutations = witness.get_permutations();
    let permutations_vec =
        <&dyn Sequence<usize>>::into_iter(permutations.deref()).collect::<Vec<_>>();
    println!(
        "Permutations: {:?} (size: {})",
        permutations_vec,
        permutations.size()
    );
    assert!(permutations_vec == p);

    // print assignment
    let assignment = witness.get_assignment();
    let assignment_vec = <&dyn Sequence<Vec<usize>>>::into_iter(assignment.deref())
        .into_iter()
        .collect::<Vec<_>>();
    println!(
        "Assignment: {:?} (size: {})",
        assignment_vec,
        assignment.size()
    );

    assert!(assignment_vec == a);
}
