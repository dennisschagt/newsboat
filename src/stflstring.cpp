#include "stflstring.h"

#include "utils.h"

namespace newsboat {

StflString::StflString(const std::string& stfl_quoted)
	: quoted_string(stfl_quoted)
{
}

StflString StflString::from_regular(const std::string& str)
{
	std::string quoted = utils::replace_all(str, "<", "<>");
	return StflString(quoted);
}

StflString StflString::operator+(const StflString& rhs)
{
	return StflString(this->quoted_string + rhs.get_stfl_quoted_string());
}

std::string StflString::get_stfl_quoted_string() const
{
	return quoted_string;
}

} // namespace newsboat
