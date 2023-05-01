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

size_t sequence_color_size(const SequenceColor &self) {
  return self.seq.size();
}

rust::Vec<size_t> sequence_color_get_index(const SequenceColor &self,
                                           size_t idx) {
  auto val = self.seq.getElementByIndex(idx);
  rust::Vec<size_t> ret;
  ret.reserve(val.size());
  for (size_t i = 0, size = val.size(); i < size; i++) {
    // NOTE: FieldElement can be represented as size_t, which
    // significantly simplifies the work and saves time.
    // Instead of returning Vec<FieldElement>, map FieldElement to primitive
    // type (size_t), and return it.
    size_t m = Algebra::mapFieldElementToInteger(0, 64, val.at(i));
    ret.push_back(m);
  }

  return ret;
}

} // namespace libstark
