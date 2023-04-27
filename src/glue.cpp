#include "glue.hpp"
#include "Element.h"
#include "common/langCommon/Sequence.hpp"
#include "languages/Bair/BairWitness.hpp"
#include "languages/Bair/BairWitnessChecker.hpp"
#include "workspace/src/glue.rs.h"

// we are building in C++11, it doesn't have make_unique...
template <typename T, typename... Args>
std::unique_ptr<T> make_unique(Args &&...args) {
  return std::unique_ptr<T>(new T(std::forward<Args>(args)...));
}

namespace libstark {
template <typename T> class Vec2Sequence : public Sequence<T> {
private:
  std::vector<T> _data;

public:
  using base = Sequence<T>;
  ~Vec2Sequence() override = default;
  Vec2Sequence(std::vector<T> data) : _data(std::move(data)) {}
  T getElementByIndex(typename base::index_t index) const override {
    return _data.at(index);
  }
};

// this class is inside .cpp UT file.
// it should normally be exported through a separate hpp/cpp pair,
// copy it here for a temp quickfix.
class allwaysSatisfiedSystem : public ConstraintSys {
public:
  allwaysSatisfiedSystem(size_t numVars)
      : numVars_(numVars), firstUsed_(numVars == 2){};
  size_t numVars() const { return numVars_; }
  allwaysSatisfiedSystem *clone() const {
    return new allwaysSatisfiedSystem(numVars_);
  }
  const polySet_t &constraints() const { return noPolys_; }
  bool varUsed(const size_t varId) const {
    // verify the case where first var is
    // not routed, is handled
    if ((varId == 0) && !firstUsed_)
      return false;

    // non trivial function
    // this is implemented this way
    // so the embedding itself will be non trivial
    // This should simulate the expected case
    // where the first half of the variables belongs the
    // previous configuration + auxiliary variables used when
    //"previous configuration is next configuration"
    // and the second half is the next configuration
    // and the auxiliary variables relevant to current check.
    // so the constraint system would probably say that it uses
    // only previous configuration,next configuration, and
    // auxiliary variables relevant to current check.
    return (varId <= (numVars_ * 0.75));
  }

private:
  size_t numVars_;
  const polySet_t noPolys_; // empty set
  const bool firstUsed_;
};

// this function is inside the test cpp, it should be extracted to a separate
// hpp/cpp. as a temp quickfix, copy it here.
static std::vector<Algebra::FieldElement>
getRandomPadding(const size_t &vectorLen) {
  std::vector<Algebra::FieldElement> res(vectorLen);
  for (auto &x : res)
    x = Algebra::generateRandom();
  return res;
}

static inline BairWitness::color_t make_color(const SharedColor &c) {
  std::vector<Algebra::FieldElement> elements;
  elements.reserve(c.v.size());
  for (const auto &i : c.v) {
    // map size_t into Algebra::FieldElement
    Algebra::FieldElement fe = Algebra::mapIntegerToFieldElement(0, 64, i);
    elements.push_back(std::move(fe));
  }
  return BairWitness::color_t{elements.begin(), elements.end()};
}

std::shared_ptr<BairWitness>
new_bair_witness(const rust::Vec<SharedColor> &assignment,
                 const rust::Vec<size_t> &permutation) {
  // we got rust::Vec<SharedColor>, where SharedColor is rust::Vec<uint64_t>.
  // we need to cast it to std::vector<std::vector<uint64_t>>
  std::vector<BairWitness::color_t> assignment_vec;
  assignment_vec.reserve(assignment.size());
  for (const auto &c : assignment) {
    assignment_vec.push_back(std::move(make_color(c)));
  }

  BairWitness::assignment_ptr A =
      make_unique<Vec2Sequence<BairWitness::color_t>>(move(assignment_vec));
  BairWitness::permutation_ptr B = make_unique<Vec2Sequence<size_t>>(
      std::move(std::vector<size_t>{permutation.begin(), permutation.end()}));

  return std::make_shared<BairWitness>(std::move(A), std::move(B));
}

std::shared_ptr<BairInstance>
new_hardcoded_bair_instance(size_t vector_len, int16_t domain_size_indicator) {
  size_t domain_size = size_t(1) << domain_size_indicator;

  // empty constraints
  BairInstance::constraintsPtr_t constraintsAssignment(
      new allwaysSatisfiedSystem(vector_len * 2));
  BairInstance::constraintsPtr_t constraintsPermutation(
      new allwaysSatisfiedSystem(vector_len * 2));

  // construct the instance
  BairInstance instance(
      vector_len, domain_size_indicator, move(constraintsAssignment),
      move(constraintsPermutation), BairInstance::boundaryConstraints_t(),
      getRandomPadding(vector_len));
  return std::make_shared<BairInstance>(std::move(instance));
}

bool bair_witness_checker_verify_constraints_assignment(
    std::shared_ptr<BairInstance> instance,
    std::shared_ptr<BairWitness> witness) {
  return BairWitnessChecker::verify_constraintsAssignment(*instance, *witness);
}

bool bair_witness_checker_verify_constraints_permutation(
    std::shared_ptr<BairInstance> instance,
    std::shared_ptr<BairWitness> witness) {
  return BairWitnessChecker::verify_constraintsPermutation(*instance, *witness);
}
} // namespace libstark
