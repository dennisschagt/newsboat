use libnewsboat::{matchable, matcher, matchererror};
use std::pin::Pin;

// cxx doesn't allow to share types from other crates, so we have to wrap it
// cf. https://github.com/dtolnay/cxx/issues/496
struct Matcher(matcher::Matcher);

struct ParseResult {
    result: Result<matcher::Matcher, String>,
}

struct MatchResult {
    result: Result<bool, matchererror::MatcherError>,
}

#[cxx::bridge(namespace = "newsboat::matcher::bridged")]
mod bridged {
    #[namespace = "newsboat::matchererror::bridged"]
    extern "C++" {
        include!("libnewsboat-ffi/src/matchererror.rs.h");
        type MatcherError = crate::matchererror::MatcherError;
    }

    #[namespace = "newsboat"]
    unsafe extern "C++" {
        include!("matchable.h");

        type Matchable;

        fn get_matchable_attribute(
            matchable: Pin<&Matchable>,
            attribute: &str,
            output: &mut String,
        ) -> bool;
    }

    extern "Rust" {
        type Matcher;
        type MatchResult;
        type ParseResult;

        fn parse(input: &str) -> Box<ParseResult>;
        fn parse_succeeded(result: &ParseResult) -> bool;
        fn parse_error(result: Box<ParseResult>) -> String;
        fn parsed_matcher(result: Box<ParseResult>) -> Box<Matcher>;

        fn get_expression(matcher: &Matcher) -> &str;
        fn matches(matcher: &Matcher, matchable: Pin<&Matchable>) -> Box<MatchResult>;
        fn matching_succeeded(result: &MatchResult) -> bool;
        fn match_error(result: Box<MatchResult>) -> Box<MatcherError>;
        fn match_result(result: Box<MatchResult>) -> bool;
    }
}

fn parse(input: &str) -> Box<ParseResult> {
    Box::new(ParseResult {
        result: matcher::Matcher::parse(input),
    })
}

fn parse_succeeded(result: &ParseResult) -> bool {
    result.result.is_ok()
}

fn parse_error(result: Box<ParseResult>) -> String {
    result.result.err().unwrap()
}

fn parsed_matcher(result: Box<ParseResult>) -> Box<Matcher> {
    Box::new(Matcher(result.result.unwrap()))
}

fn get_expression(matcher: &Matcher) -> &str {
    matcher.0.get_expression()
}

fn matches(matcher: &Matcher, matchable: Pin<&bridged::Matchable>) -> Box<MatchResult> {
    struct MatchableAdapter<'a> {
        pub matchable: Pin<&'a bridged::Matchable>,
    }

    impl<'a> matchable::Matchable for MatchableAdapter<'a> {
        fn attribute_value(&self, attr: &str) -> Option<String> {
            let mut value = String::new();
            if bridged::get_matchable_attribute(self.matchable, &attr, &mut value) {
                Some(value)
            } else {
                None
            }
        }
    }

    let item = MatchableAdapter { matchable };

    Box::new(MatchResult {
        result: matcher.0.matches(&item),
    })
}

fn matching_succeeded(result: &MatchResult) -> bool {
    result.result.is_ok()
}

fn match_error(result: Box<MatchResult>) -> Box<crate::matchererror::MatcherError> {
    Box::new(crate::matchererror::MatcherError(
        result.result.err().unwrap(),
    ))
}

fn match_result(result: Box<MatchResult>) -> bool {
    result.result.unwrap()
}
