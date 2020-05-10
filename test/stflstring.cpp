#include "stflstring.h"

#include <string>
#include <utility>
#include <vector>

#include "3rd-party/catch.hpp"

using namespace newsboat;

TEST_CASE("Construct from string which is already stfl-encoded", "[StflString]")
{
	const std::vector<std::string> testStrings {
		"hello world!",
		"less than: <",
		"greater than: >",
		"<<<combined>>>"
	};

	SECTION("string should not be altered in any way") {
		for (const auto& str : testStrings) {
			const auto stflString = StflString(str);
			REQUIRE(stflString.get_stfl_quoted_string() == str);
		}
	}
}

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

	SECTION("all occurences of `<` should be replaced by `<>`") {
		for (const auto& pair : testStrings) {
			const auto stflString = StflString::from_regular(pair.first);
			REQUIRE(stflString.get_stfl_quoted_string() == pair.second);
		}
	}
}

TEST_CASE("Append multiple StflString objects", "[StflString]")
{
	auto first = StflString("<hello>");
	auto second = StflString::from_regular(" escaped <<word>>");
	auto third = StflString(" world");

	SECTION("adding two StflString objects") {
		auto combined = first + third;
		REQUIRE(combined.get_stfl_quoted_string() == "<hello> world");
	}

	SECTION("adding three StflString objects") {
		auto combined = first + second + third;
		REQUIRE(combined.get_stfl_quoted_string() ==
			"<hello> escaped <><>word>> world");
	}
}
