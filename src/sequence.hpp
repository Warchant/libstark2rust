#pragma once

#include "common/langCommon/Sequence.hpp"
#include "languages/Bair/BairWitness.hpp"
#include <cstddef>
#include <rust/cxx.h> // for rust::Vec

namespace libstark {

struct SequenceUsize {
  const Sequence<size_t> &seq;
};
size_t sequence_usize_get_index(const SequenceUsize &self, size_t idx);
size_t sequence_usize_size(const SequenceUsize &self);

// struct SequenceColor {
//   Sequence<BairWitness::color_t> &seq;
// };
// std::shared_ptr<std::vector<Algebra::FieldElement>> sequence_color_get_index(const SequenceColor &self, size_t idx);
// size_t sequence_color_size(const SequenceColor &self);

} // namespace libstark
