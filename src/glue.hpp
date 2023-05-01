#pragma once

#include "common/langCommon/Sequence.hpp"
#include <cstdint>
#include <memory>
#include <rust/cxx.h>
#include <vector>
#include "workspace/src/glue.rs.h"
#include "sequence.hpp"

namespace libstark {

std::shared_ptr<BairWitness>
new_bair_witness(const rust::Vec<SharedColor> &assignment,
                 const rust::Vec<size_t> &permutation);


std::shared_ptr<BairInstance>
new_hardcoded_bair_instance(size_t vector_len, int16_t domain_size_indicator);

bool bair_witness_checker_verify_constraints_assignment(
    std::shared_ptr<BairInstance> instance,
    std::shared_ptr<BairWitness> witness);

bool bair_witness_checker_verify_constraints_permutation(
    std::shared_ptr<BairInstance> instance,
    std::shared_ptr<BairWitness> witness);


std::shared_ptr<SequenceUsize> bair_witness_get_permutation(const BairWitness& witness);
std::shared_ptr<SequenceColor> bair_witness_get_assignment(const BairWitness& witness);

} // namespace libstark
