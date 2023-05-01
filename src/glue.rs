use std::ops::Deref;

#[allow(unused_imports)]
#[allow(dead_code)]
use cxx::{CxxVector, ExternType, SharedPtr, UniquePtr};
use rand::{seq::SliceRandom, Rng};

use self::{libstark::{
    new_bair_witness, new_hardcoded_bair_instance, sequence_usize_get_index, sequence_usize_size,
    BairInstance, BairWitness, SequenceUsize, SharedColor, bair_witness_get_permutation
}};

// #[cxx::bridge(namespace = "Algebra")]
// pub mod Algebra {
//     unsafe extern "C++" {
//         type FieldElement;
//     }
// }

#[cxx::bridge(namespace = "libstark")]
pub mod libstark {
    struct SharedColor {
        v: Vec<u64>,
    }

    unsafe extern "C++" {
        include!("sequence.hpp");
        include!("glue.hpp");
        include!("languages/Bair/BairWitness.hpp");
        include!("languages/Bair/BairInstance.hpp");

        type BairInstance;
        type BairWitness;

        type SequenceUsize;
        fn sequence_usize_get_index(seq: &SequenceUsize, idx: usize) -> usize;
        fn sequence_usize_size(seq: &SequenceUsize) -> usize;

        // type SequenceColor;
        // fn sequence_color_get_index(
        //     seq: &SequenceColor,
        //     idx: usize,
        // ) -> SharedPtr<CxxVector<FieldElement>>;
        // fn sequence_color_size(seq: &SequenceColor) -> usize;

        fn bair_witness_get_permutation(witness: &BairWitness) -> SharedPtr<SequenceUsize>;

        ///////////////////////////////////////////////////////////////
        fn new_bair_witness(
            assignment: &Vec<SharedColor>,
            permutation: &Vec<usize>,
        ) -> SharedPtr<BairWitness>;

        fn new_hardcoded_bair_instance(
            vector_len: usize,
            // other fields can also be wrapped, it just takes more time
            domain_size_indicator: i16,
        ) -> SharedPtr<BairInstance>;

        fn bair_witness_checker_verify_constraints_assignment(
            instance: SharedPtr<BairInstance>,
            witness: SharedPtr<BairWitness>,
        ) -> bool;

        fn bair_witness_checker_verify_constraints_permutation(
            instance: SharedPtr<BairInstance>,
            witness: SharedPtr<BairWitness>,
        ) -> bool;
    }
}

// create BairInstance and BairWitness in rust
pub fn generate_valid_constraints() -> (SharedPtr<BairInstance>, SharedPtr<BairWitness>) {
    let vector_len: usize = 3;
    let domain_size_indicator: i16 = 3;
    let domain_size: usize = (1 << domain_size_indicator) - 1;
    let mut rng = rand::thread_rng();

    // generate random assignment
    let a = (0..domain_size)
        .map(|_| SharedColor {
            v: (0..vector_len).map(|_| rng.gen::<u64>()).collect(),
        })
        .collect();

    // generate random permutation
    let mut p: Vec<usize> = (0..domain_size).collect();
    p.shuffle(&mut rng);

    let witness = new_bair_witness(&a, &p);
    let instance = new_hardcoded_bair_instance(vector_len, domain_size_indicator);
    return (instance, witness);
}


impl BairWitness {
    pub fn get_permutations(&self) -> SharedPtr<SequenceUsize> {
        bair_witness_get_permutation(&self)
    }
}
