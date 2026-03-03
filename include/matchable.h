#ifndef NEWSBOAT_MATCHABLE_H_
#define NEWSBOAT_MATCHABLE_H_

#include <optional>
#include <string>

#include "rust/cxx.h"

namespace newsboat {

class Matchable {
public:
	Matchable() = default;
	virtual ~Matchable() = default;
	virtual std::optional<std::string> attribute_value(const std::string& attr)
	const =
		0;
};

// For Rust FFI
bool get_matchable_attribute(const Matchable& matchable, rust::Str attribute,
	rust::String& output);

} // namespace newsboat

#endif /* NEWSBOAT_MATCHABLE_H_ */

