#include "matcherexception.h"

#include "3rd-party/catch.hpp"

#include <cstring>
#include <map>

#include "matchable.h"

#include "libnewsboat-ffi/src/matcher.rs.h"
#include "libnewsboat-ffi/src/matchererror.rs.h"

using namespace newsboat;

namespace {
class MatchableMock : public Matchable {
public:
	MatchableMock(std::map<std::string, std::string> attributes)
		: attributes(attributes)
	{
	}
	~MatchableMock() override = default;

	std::optional<std::string> attribute_value(const std::string& attr) const override
	{
		auto it = attributes.find(attr);
		if (it != attributes.end()) {
			return it->second;
		} else {
			return std::nullopt;
		}
	}

private:
	std::map<std::string, std::string> attributes;
};
}

TEST_CASE("Can be constructed from Rust error returned over FFI",
	"[MatcherException]")
{
	SECTION("Attribute unavailable") {
		const auto e = MatcherException::from_rust_error(
				*matchererror::bridged::get_test_attr_unavail_error());
		REQUIRE(e.type() == MatcherException::Type::AttributeUnavailable);
		REQUIRE(e.info() == "test_attribute");
		REQUIRE(e.info2().empty());
		REQUIRE_FALSE(strlen(e.what()) == 0);
	}

	SECTION("Invalid regex") {
		const auto e = MatcherException::from_rust_error(
				*matchererror::bridged::get_test_invalid_regex_error());
		REQUIRE(e.type() == MatcherException::Type::InvalidRegex);
		REQUIRE(e.info() == "?!");
		REQUIRE(e.info2() == "inconceivable happened!");
		REQUIRE_FALSE(strlen(e.what()) == 0);
	}

	SECTION("Actual rust matcher") {
		SECTION("parsing failure") {
			std::string invalidInput = "";
			auto parseResult = matcher::bridged::parse(invalidInput);
			REQUIRE_FALSE(matcher::bridged::parse_succeeded(*parseResult));
			auto error = matcher::bridged::parse_error(std::move(parseResult));
			REQUIRE_FALSE(error.empty());

			// TODO: Remove (to avoid test failures in case of translated error message)
			REQUIRE(error == "Parse error at position 0: expected attribute name");
		}

		SECTION("valid parse") {
			auto parseResult = matcher::bridged::parse(R"(title =~ "newsboat")");
			REQUIRE(matcher::bridged::parse_succeeded(*parseResult));
			auto matcher = matcher::bridged::parsed_matcher(std::move(parseResult));

			SECTION("no match") {
				MatchableMock matchable(std::map<std::string, std::string> {
					{"title", "test title"},
				});
				auto matchResult = matcher::bridged::matches(*matcher, matchable);
				REQUIRE(matcher::bridged::matching_succeeded(*matchResult));
				REQUIRE_FALSE(matcher::bridged::match_result(std::move(matchResult)));
			}

			SECTION("match") {
				MatchableMock matchable(std::map<std::string, std::string> {
					{"title", "test newsboat title"},
				});
				auto matchResult = matcher::bridged::matches(*matcher, matchable);
				REQUIRE(matcher::bridged::matching_succeeded(*matchResult));
				REQUIRE(matcher::bridged::match_result(std::move(matchResult)));
			}

			SECTION("match failure: attribute unavailable") {
				MatchableMock matchable({});
				auto matchResult = matcher::bridged::matches(*matcher, matchable);
				REQUIRE_FALSE(matcher::bridged::matching_succeeded(*matchResult));
				auto matchError = matcher::bridged::match_error(std::move(matchResult));
				auto matchErrorFfi = matchererror::bridged::matcher_error_to_ffi(*matchError);
				REQUIRE(matchErrorFfi.err_type == MatcherException::Type::AttributeUnavailable);
				REQUIRE(!matchErrorFfi.info.empty());
				REQUIRE(matchErrorFfi.info2.empty());
			}
		}

		SECTION("match failure: invalid regex") {
			auto parseResult = matcher::bridged::parse(R"(title =~ "invalid regex[")");
			REQUIRE(matcher::bridged::parse_succeeded(*parseResult));
			auto matcher = matcher::bridged::parsed_matcher(std::move(parseResult));
			MatchableMock matchable(std::map<std::string, std::string> {
				{"title", "test newsboat title"},
			});
			auto matchResult = matcher::bridged::matches(*matcher, matchable);
			REQUIRE_FALSE(matcher::bridged::matching_succeeded(*matchResult));
			auto matchError = matcher::bridged::match_error(std::move(matchResult));
			auto matchErrorFfi = matchererror::bridged::matcher_error_to_ffi(*matchError);
			REQUIRE(matchErrorFfi.err_type == MatcherException::Type::InvalidRegex);
			REQUIRE(!matchErrorFfi.info.empty());
			REQUIRE(!matchErrorFfi.info2.empty());
		}
	}
}
