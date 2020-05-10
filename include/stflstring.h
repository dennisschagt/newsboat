#ifndef NEWSBOAT_STFLSTRING_H_
#define NEWSBOAT_STFLSTRING_H_

#include <string>

namespace newsboat {

class StflString {
public:
	explicit StflString(const std::string& stfl_quoted);
	static StflString from_regular(const std::string& str);

	StflString operator+(const StflString& rhs);

	std::string get_stfl_quoted_string() const;
private:

	std::string quoted_string;

};

} // namespace newsboat

#endif /* NEWSBOAT_STFLSTRING_H_ */
