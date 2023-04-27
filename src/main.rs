use glue::generate_valid_constraints;

use crate::glue::libstark::{
    bair_witness_checker_verify_constraints_assignment,
    bair_witness_checker_verify_constraints_permutation,
};

mod glue;

fn main() {
    let (instance, witness) = generate_valid_constraints();
    let a = bair_witness_checker_verify_constraints_assignment(instance.clone(), witness.clone());
    let b = bair_witness_checker_verify_constraints_permutation(instance.clone(), witness.clone());

    println!("{}", a);
    println!("{}", b);

    assert!(a);
    assert!(b);
}
