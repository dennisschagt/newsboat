#include "matchable.h"

namespace newsboat {

bool get_matchable_attribute(const Matchable& matchable, rust::Str attribute,
	rust::String& output)
{
	const auto value = matchable.attribute_value(std::string(attribute));
	if (value.has_value()) {
		output = value.value();
		return true;
	}
	return false;
}

}
