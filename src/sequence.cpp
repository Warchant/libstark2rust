#include "sequence.hpp"
#include "languages/Bair/BairWitness.hpp"
#include <cassert>

namespace libstark {

size_t sequence_usize_get_index(const SequenceUsize &self, size_t idx) {
  return self.seq.getElementByIndex(idx);
}

size_t sequence_usize_size(const SequenceUsize &self) {
  return self.seq.size();
}

// // cxx is not able to return std::vector by value, so we allocate
// // Vec in Rust, pass its reference to C++, then C++ fills it
// // with data it wants to return.
// std::shared_ptr<std::vector<Algebra::FieldElement>> sequence_color_get_index(const SequenceColor &self, size_t idx) {
//   auto val = self.seq.getElementByIndex(idx);
//   return std::make_shared<decltype(val)>(std::move(val));
// }

size_t sequence_color_size(const SequenceColor &self) {
  return self.seq.size();
}

} // namespace libstark
