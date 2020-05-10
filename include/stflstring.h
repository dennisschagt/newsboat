#ifndef NEWSBOAT_STFLSTRING_H_
#define NEWSBOAT_STFLSTRING_H_

#include <map>
#include <string>

namespace newsboat {

class StflString {
public:
	explicit StflString(const std::string& regular_string);
	static StflString from_quoted(std::string quoted_str);

	StflString operator+(const StflString& rhs) const;
	std::string get_stfl_quoted_string() const;
	std::string get_raw_string() const;
	void apply_style(std::string style_name, size_t start, size_t end);

private:
	std::string str;
	std::map<size_t, std::string> style_tags;
};

} // namespace newsboat

#endif /* NEWSBOAT_STFLSTRING_H_ */
