#include "stflstring.h"

#include <string>
#include <utility>
#include <vector>

#include "3rd-party/catch.hpp"

using namespace newsboat;

TEST_CASE("Construct from regular string which is not yet encoded",
	"[StflString]")
{
	// Pairs of <"input", "expected output">
	const std::vector<std::pair<std::string, std::string>> testStrings {
		{"hello world!", "hello world!"},
		{"less than: <", "less than: <>"},
		{"greater than: >", "greater than: >"},
		{"<<<combined>>>", "<><><>combined>>>"}
	};

	SECTION("get_raw_string() should return string as-is") {
		for (const auto& pair : testStrings) {
			const auto stflString = StflString(pair.first);
			REQUIRE(stflString.get_raw_string() == pair.first);
		}
	}

	SECTION("get_stfl_quoted_string() should replace all occurences of `<` with `<>`") {
		for (const auto& pair : testStrings) {
			const auto stflString = StflString(pair.first);
			REQUIRE(stflString.get_stfl_quoted_string() == pair.second);
		}
	}
}

TEST_CASE("Construct from string which is already stfl-encoded",
	"[StflString]")
{
	const std::vector<std::string> testStrings {
		"hello world!",
		"less than: <>",
		"greater than: >",
		"<><><>combined>>>"
	};

	SECTION("get_stfl_quoted_string() should return original string") {
		for (const auto& str : testStrings) {
			const auto stflString = StflString::from_quoted(str);
			REQUIRE(stflString.get_stfl_quoted_string() == str);
		}
	}
}

TEST_CASE("Append multiple StflString objects", "[StflString]")
{
	auto first = StflString::from_quoted("<color>hello");
	auto second = StflString(" escaped <word>");
	auto third = StflString::from_quoted(" world</>");

	SECTION("adding two StflString objects") {
		auto combined = first + third;
		REQUIRE(combined.get_raw_string() == "hello world");
		REQUIRE(combined.get_stfl_quoted_string() == "<color>hello world</>");
	}

	SECTION("adding three StflString objects") {
		auto combined = first + second + third;
		REQUIRE(combined.get_raw_string() == "hello escaped <word> world");
		REQUIRE(combined.get_stfl_quoted_string() ==
			"<color>hello escaped <>word> world</>");
	}
}

TEST_CASE("Append strings with tags at the end", "[StflString]")
{
	auto first = StflString::from_quoted("<yellow>should be yellow</>");
	auto second = StflString::from_quoted("<blue>should be blue</>");

	SECTION("starting tag of second string overwrites tag at end of first string") {
		auto combined = first + second;
		std::string expected = "<yellow>should be yellow<blue>should be blue</>";
		REQUIRE(combined.get_stfl_quoted_string() == expected);
	}
}

TEST_CASE("apply_style() inserts style tags in correct locations",
	"[StflString]")
{
	StflString stflString("lorem ipsum dolor");

	REQUIRE(stflString.get_stfl_quoted_string() == "lorem ipsum dolor");

	SECTION("second apply_style() range completely within first range so it restores first tag at end") {
		stflString.apply_style("<abc>", 0, 17);
		REQUIRE(stflString.get_stfl_quoted_string() == "<abc>lorem ipsum dolor</>");

		stflString.apply_style("<def>", 6, 11);
		REQUIRE(stflString.get_stfl_quoted_string() ==
			"<abc>lorem <def>ipsum<abc> dolor</>");
	}

	SECTION("second apply_style() range completely overlaps first range so first tag is no longer present") {
		stflString.apply_style("<abc>", 6, 11);
		REQUIRE(stflString.get_stfl_quoted_string() == "lorem <abc>ipsum</> dolor");

		stflString.apply_style("<def>", 0, 17);
		REQUIRE(stflString.get_stfl_quoted_string() == "<def>lorem ipsum dolor</>");
	}
}

TEST_CASE("apply_style() restores previous tag if necessary", "[StflString]")
{
	auto stflString = StflString::from_quoted("<0>lorem<1>ipsum</>text");
	std::string tagName = "<test>";

	SECTION("end before tag switch so restore first tag") {
		stflString.apply_style(tagName, 0, 4);
		REQUIRE(stflString.get_stfl_quoted_string() == "<test>lore<0>m<1>ipsum</>text");
	}

	SECTION("end on tag switch so no need to restore anything") {
		stflString.apply_style(tagName, 0, 5);
		REQUIRE(stflString.get_stfl_quoted_string() == "<test>lorem<1>ipsum</>text");
	}

	SECTION("end after a closing tag (</>) so close at the end") {
		SECTION("on the closing tag") {
			stflString.apply_style(tagName, 0, 10);
			REQUIRE(stflString.get_stfl_quoted_string() == "<test>loremipsum</>text");
		}

		SECTION("after the closing tag") {
			stflString.apply_style(tagName, 0, 11);
			REQUIRE(stflString.get_stfl_quoted_string() == "<test>loremipsumt</>ext");
		}
	}
}
