#include "stflstring.h"

#include "utils.h"

namespace newsboat {

StflString::StflString(const std::string& regular_string)
	: str(regular_string)
	, style_tags({})
{
}

StflString StflString::from_quoted(std::string quoted_str)
{
	// Extract STFL style tags from the quoted string
	std::map<size_t, std::string> tags;
	size_t pos = 0;
	while (pos < quoted_str.size()) {
		auto tag_start = quoted_str.find_first_of("<>", pos);
		if (tag_start == std::string::npos) {
			break;
		}
		if (quoted_str[tag_start] == '>') {
			// Keep unmatched '>' (stfl way of encoding a literal '>')
			pos = tag_start + 1;
			continue;
		}
		auto tag_end = quoted_str.find_first_of("<>", tag_start + 1);
		if (tag_end == std::string::npos) {
			break;
		}
		if (quoted_str[tag_end] == '<') {
			// First '<' bracket is unmatched, ignoring it
			pos = tag_start + 1;
			continue;
		}
		if (tag_end - tag_start == 1) {
			// Convert "<>" into "<" (stfl way of encoding a literal '<')
			quoted_str.erase(tag_end, 1);
			pos = tag_start + 1;
			continue;
		}
		tags[tag_start] = quoted_str.substr(tag_start, tag_end - tag_start + 1);
		quoted_str.erase(tag_start, tag_end - tag_start + 1);
		pos = tag_start;
	}
	auto result = StflString(quoted_str);
	result.style_tags = tags;
	return result;
}

StflString StflString::operator+(const StflString& rhs) const
{
	auto result = StflString(this->str + rhs.str);
	result.style_tags = this->style_tags;

	size_t offset = this->str.length();
	for (const auto& style_tag : rhs.style_tags) {
		result.style_tags[offset + style_tag.first] = style_tag.second;
	}
	return result;
}

std::string StflString::get_stfl_quoted_string() const
{
	// Expand "<" into "<>"
	size_t pos = 0;
	std::string result = str;
	std::map<size_t, std::string> tags = style_tags;
	while (pos < result.size()) {
		auto bracket = result.find_first_of("<", pos);
		if (bracket == std::string::npos) {
			break;
		}
		pos = bracket + 1;
		// Add to the `tags` map so we don't have to shift all the positions in that map
		// (would be necessary if inserting directly into `result`
		tags[pos] = ">" + tags[pos];
	}

	for (auto it = tags.rbegin(); it != tags.rend(); ++it) {
		if (it->first > result.length()) {
			// Ignore tags outside of string
			continue;
		}
		result.insert(it->first, it->second);
	}

	return result;
}

std::string StflString::get_raw_string() const
{
	return str;
}

void StflString::apply_style(std::string style_name, size_t start, size_t end)
{
	if (end <= start) {
		return;
	}

	// Find the latest tag occurring before `end`.
	// This requires that looping executes in ascending order of location.
	std::string latest_tag = "</>";
	for (const auto& style_tag : style_tags) {
		const size_t location = style_tag.first;
		if (location > end) {
			break;
		}
		latest_tag = style_tag.second;
	}
	style_tags[start] = style_name;
	style_tags[end] = latest_tag;

	// Remove any old tags between the start and end marker
	for (auto it = style_tags.begin(); it != style_tags.end(); ) {
		if (it->first > start && it->first < end) {
			it = style_tags.erase(it);
		} else {
			++it;
		}
	}
}

} // namespace newsboat
