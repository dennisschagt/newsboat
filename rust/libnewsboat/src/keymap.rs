use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, tag_no_case, take},
    character::complete::{satisfy, space0, space1},
    combinator::{complete, eof, map, recognize, value, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn unquoted_token(input: &str) -> IResult<&str, String> {
    let parser = map(recognize(is_not(" ;")), String::from);
    let mut parser = verify(parser, |t: &str| t != "--");

    parser(input)
}

fn quoted_token<'a>(input: &'a str) -> IResult<&'a str, String> {
    let parser = escaped_transform(is_not(r#""\"#), '\\', |control_char: &'a str| {
        alt((
            value(r#"""#, tag(r#"""#)),
            value(r#"\"#, tag(r#"\"#)),
            value("\r", tag("r")),
            value("\n", tag("n")),
            value("\t", tag("t")),
            take(1usize), // all other escaped characters are passed through, unmodified
        ))(control_char)
    });

    let double_quote = tag("\"");
    let mut parser = delimited(&double_quote, parser, alt((&double_quote, eof)));

    parser(input)
}

fn token(input: &str) -> IResult<&str, String> {
    let mut parser = alt((quoted_token, unquoted_token));
    parser(input)
}

fn operation_with_args(input: &str) -> IResult<&str, Vec<String>> {
    let mut parser = separated_list1(space1, token);
    parser(input)
}

fn semicolon(input: &str) -> IResult<&str, &str> {
    delimited(space0, tag(";"), space0)(input)
}

fn operation_sequence(input: &str) -> IResult<&str, Vec<Vec<String>>> {
    let parser = separated_list0(many1(semicolon), operation_with_args);
    let parser = delimited(many0(semicolon), parser, many0(semicolon));
    let parser = preceded(space0, parser);

    let mut parser = complete(parser);

    parser(input)
}

fn operation_description(input: &str) -> IResult<&str, &str> {
    let start_token = delimited(space0, tag("--"), space0);

    let double_quote = tag("\"");
    let parser = delimited(&double_quote, recognize(is_not("\"")), &double_quote);

    let mut parser = preceded(start_token, parser);

    parser(input)
}

#[derive(Debug)]
pub struct Key {
    pub key: String,
    pub shift: bool,
    pub control: bool,
    pub meta: bool,
}

impl Key {
    fn regular_key(key: char) -> Key {
        Key {
            key: key.to_lowercase().to_string(),
            shift: key.is_uppercase(),
            control: false,
            meta: false,
        }
    }

    fn control_key(key: char) -> Key {
        Key {
            key: key.to_string(),
            shift: false,
            control: true,
            meta: false,
        }
    }

    fn named_key(key: String) -> Key {
        Key {
            key: key,
            shift: false,
            control: false,
            meta: false,
        }
    }
}

fn single_key(input: &str) -> IResult<&str, char> {
    let parser = satisfy(|c| c != '<' && c != '>' && c != '^');

    parser(input)
}

fn regular_key(input: &str) -> IResult<&str, Key> {
    let parser = single_key;
    let mut parser = map(parser, |key| Key::regular_key(key));

    parser(input)
}

fn control_key(input: &str) -> IResult<&str, Key> {
    let parser = satisfy(|c| c.is_alphabetic());
    let parser = preceded(tag("^"), parser);
    let mut parser = map(parser, |key| Key::control_key(key));

    parser(input)
}

fn named_key(input: &str) -> IResult<&str, String> {
    let parser = alt((
        tag_no_case("space"),
        tag_no_case("enter"),
        tag_no_case("up"),
        tag_no_case("down"),
        tag_no_case("left"),
        tag_no_case("right"),
        tag_no_case("esc"),
        tag_no_case("plus"),
        tag_no_case("minus"),
        tag_no_case("lt"),
        tag_no_case("gt"),
    ));

    let mut parser = map(parser, |key: &str| key.to_lowercase());

    parser(input)
}

fn named_key_tag(input: &str) -> IResult<&str, Key> {
    let parser = delimited(tag("<"), named_key, tag(">"));
    let mut parser = map(parser, |key_name| Key::named_key(key_name));

    parser(input)
}

/// Parses a key with modifiers in `<C-S-M-d>` format
fn modifiers_key_tag(input: &str) -> IResult<&str, Key> {
    let single_key = map(single_key, |c| c.to_string());

    let parser = alt((tag("C"), tag("S"), tag("M")));
    let parser = terminated(parser, tag("-"));
    let parser = many1(parser);
    let parser = tuple((parser, alt((single_key, named_key))));
    let parser = delimited(tag("<"), parser, tag(">"));

    let mut parser = map(parser, |(modifiers, key)| Key {
        key,
        shift: modifiers.contains(&"S"),
        control: modifiers.contains(&"C"),
        meta: modifiers.contains(&"M"),
    });

    parser(input)
}

fn key_sequence(input: &str) -> IResult<&str, Vec<Key>> {
    let mut parser = many1(alt((
        regular_key,
        control_key,
        named_key_tag,
        modifiers_key_tag,
    )));

    parser(input)
}

/// Split a semicolon-separated list of operations into a vector. Each operation is represented by
/// a non-empty sub-vector, where the first element is the name of the operation, and the rest of
/// the elements are operation's arguments.
///
/// Tokens can be double-quoted. Such tokens can contain spaces and C-like escaped sequences: `\n`
/// for newline, `\r` for carriage return, `\t` for tab, `\"` for double quote, `\\` for backslash.
/// Unsupported sequences are stripped of the escaping, e.g. `\e` turns into `e`.
///
/// This function assumes that the input string:
/// 1. doesn't contain a comment;
/// 2. doesn't contain backticks that need to be processed.
///
/// Returns `None` if the input could not be parsed.
pub fn tokenize_operation_sequence(input: &str) -> Option<(Vec<Vec<String>>, &str)> {
    match operation_sequence(input) {
        Ok((leftovers, tokens)) => Some((tokens, leftovers)),
        Err(_error) => None,
    }
}

pub fn tokenize_operation_description(input: &str) -> Option<&str> {
    match operation_description(input) {
        Ok((_leftovers, description)) => Some(description),
        Err(_error) => None,
    }
}

pub fn tokenize_key_sequence(input: &str) -> Option<Vec<Key>> {
    match key_sequence(input) {
        Ok((_leftovers, keys)) => Some(keys),
        Err(_error) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::tokenize_key_sequence;
    use super::tokenize_operation_description;
    use super::tokenize_operation_sequence;

    #[test]
    fn t_tokenize_operation_sequence_works_for_all_cpp_inputs() {
        assert_eq!(
            tokenize_operation_sequence("").unwrap().0,
            Vec::<Vec<String>>::new()
        );
        assert_eq!(
            tokenize_operation_sequence("open").unwrap().0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence("open-all-unread-in-browser-and-mark-read")
                .unwrap()
                .0,
            vec![vec!["open-all-unread-in-browser-and-mark-read"]]
        );
        assert_eq!(
            tokenize_operation_sequence("; ; ; ;").unwrap().0,
            Vec::<Vec<String>>::new()
        );
        assert_eq!(
            tokenize_operation_sequence("open ; next").unwrap().0,
            vec![vec!["open"], vec!["next"]]
        );
        assert_eq!(
            tokenize_operation_sequence("open ; next ; prev").unwrap().0,
            vec![vec!["open"], vec!["next"], vec!["prev"]]
        );
        assert_eq!(
            tokenize_operation_sequence("open ; next ; prev ; quit")
                .unwrap()
                .0,
            vec![vec!["open"], vec!["next"], vec!["prev"], vec!["quit"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"set "arg 1""#).unwrap().0,
            vec![vec!["set", "arg 1"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"set "arg 1" ; set "arg 2" "arg 3""#)
                .unwrap()
                .0,
            vec![vec!["set", "arg 1"], vec!["set", "arg 2", "arg 3"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"set browser "firefox"; open-in-browser"#)
                .unwrap()
                .0,
            vec![vec!["set", "browser", "firefox"], vec!["open-in-browser"]]
        );
        assert_eq!(
            tokenize_operation_sequence("set browser firefox; open-in-browser")
                .unwrap()
                .0,
            vec![vec!["set", "browser", "firefox"], vec!["open-in-browser"]]
        );
        assert_eq!(
            tokenize_operation_sequence("open-in-browser; quit")
                .unwrap()
                .0,
            vec![vec!["open-in-browser"], vec!["quit"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"open; set browser "firefox --private-window"; quit"#)
                .unwrap()
                .0,
            vec![
                vec!["open"],
                vec!["set", "browser", "firefox --private-window"],
                vec!["quit"]
            ]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"open ;set browser "firefox --private-window" ;quit"#)
                .unwrap()
                .0,
            vec![
                vec!["open"],
                vec!["set", "browser", "firefox --private-window"],
                vec!["quit"]
            ]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"open;set browser "firefox --private-window";quit"#)
                .unwrap()
                .0,
            vec![
                vec!["open"],
                vec!["set", "browser", "firefox --private-window"],
                vec!["quit"]
            ]
        );
        assert_eq!(
            tokenize_operation_sequence("; ;; ; open",).unwrap().0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence(";;; ;; ; open",).unwrap().0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence(";;; ;; ; open ;",).unwrap().0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence(";;; ;; ; open ;; ;",)
                .unwrap()
                .0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence(";;; ;; ; open ; ;;;;",)
                .unwrap()
                .0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence(";;; open ; ;;;;",).unwrap().0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence("; open ;; ;; ;",).unwrap().0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence("open ; ;;; ;;",).unwrap().0,
            vec![vec!["open"]]
        );
        assert_eq!(
            tokenize_operation_sequence(
                r#"set browser "sleep 3; do-something ; echo hi"; open-in-browser"#
            )
            .unwrap()
            .0,
            vec![
                vec!["set", "browser", "sleep 3; do-something ; echo hi"],
                vec!["open-in-browser"]
            ]
        );
    }

    #[test]
    fn t_tokenize_operation_sequence_ignores_escaped_sequences_outside_double_quotes() {
        assert_eq!(
            tokenize_operation_sequence(r#"\t"#).unwrap().0,
            vec![vec![r#"\t"#]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"\r"#).unwrap().0,
            vec![vec![r#"\r"#]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"\n"#).unwrap().0,
            vec![vec![r#"\n"#]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"\v"#).unwrap().0,
            vec![vec![r#"\v"#]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"\""#).unwrap().0,
            vec![vec![r#"\""#]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#"\\"#).unwrap().0,
            vec![vec![r#"\\"#]]
        );
    }

    #[test]
    fn t_tokenize_operation_sequence_expands_escaped_sequences_inside_double_quotes() {
        assert_eq!(
            tokenize_operation_sequence(r#""\t""#).unwrap().0,
            vec![vec!["\t"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\r""#).unwrap().0,
            vec![vec!["\r"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\n""#).unwrap().0,
            vec![vec!["\n"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\"""#).unwrap().0,
            vec![vec!["\""]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\\""#).unwrap().0,
            vec![vec!["\\"]]
        );
    }

    #[test]
    fn t_tokenize_operation_sequence_passes_through_unsupported_escaped_chars_inside_double_quotes()
    {
        assert_eq!(
            tokenize_operation_sequence(r#""\1""#).unwrap().0,
            vec![vec!["1"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\W""#).unwrap().0,
            vec![vec!["W"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\b""#).unwrap().0,
            vec![vec!["b"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\d""#).unwrap().0,
            vec![vec!["d"]]
        );
        assert_eq!(
            tokenize_operation_sequence(r#""\x""#).unwrap().0,
            vec![vec!["x"]]
        );
    }

    #[test]
    fn t_tokenize_operation_sequence_implicitly_closes_double_quotes_at_end_of_input() {
        assert_eq!(
            tokenize_operation_sequence(r#"set "arg 1"#).unwrap().0,
            vec![vec!["set", "arg 1"]]
        );
    }

    #[test]
    fn t_tokenize_operation_sequence_allows_single_character_unquoted() {
        assert_eq!(
            tokenize_operation_sequence(r#"set a b"#).unwrap().0,
            vec![vec!["set", "a", "b"]]
        );
    }

    #[test]
    fn t_tokenize_operation_sequence_does_not_consumpe_dashdash() {
        let (operations, leftover) =
            tokenize_operation_sequence(r#"set a b -- "name of function""#).unwrap();
        assert_eq!(operations, vec![vec!["set", "a", "b"]]);
        assert_eq!(leftover, r#" -- "name of function""#);
    }

    #[test]
    fn t_tokenize_operation_sequence_allows_dashdash_in_quoted() {
        let (operations, leftover) =
            tokenize_operation_sequence(r#"set a b "--" "name of function""#).unwrap();
        assert_eq!(
            operations,
            vec![vec!["set", "a", "b", "--", "name of function"]]
        );
        assert_eq!(leftover, "");
    }

    #[test]
    fn t_tokenize_operation_description_supports_empty_input() {
        assert_eq!(tokenize_operation_description(""), None);
    }

    #[test]
    fn t_tokenize_operation_description_ignores_preceding_spaces() {
        assert_eq!(
            tokenize_operation_description(r#"-- "description""#).unwrap(),
            "description"
        );
        assert_eq!(
            tokenize_operation_description(r#"     -- "description""#).unwrap(),
            "description"
        );
    }

    #[test]
    fn t_tokenize_operation_description_ignores_trailing_spaces() {
        assert_eq!(
            tokenize_operation_description(r#"-- "description"   "#).unwrap(),
            "description"
        );
    }

    #[test]
    fn t_tokenize_operation_description_includes_whitespace() {
        assert_eq!(
            tokenize_operation_description(r#"-- "multi-word description""#).unwrap(),
            "multi-word description"
        );
        assert_eq!(
            tokenize_operation_description(r#"-- "  leading and trailing  ""#).unwrap(),
            "  leading and trailing  "
        );
    }

    #[test]
    fn t_tokenize_operation_description_requires_closing_quote() {
        assert_eq!(
            tokenize_operation_description(r#"-- "description not closed "#),
            None
        );
    }

    #[test]
    fn t_tokenize_key_sequence_requires_at_least_1_key() {
        assert!(tokenize_key_sequence("").is_none());
    }

    #[test]
    fn t_tokenize_key_sequence_parses_regular_keys() {
        let x = tokenize_key_sequence("a").unwrap();
        assert_eq!(x.len(), 1);
        assert_eq!(x[0].key, "a");
        assert_eq!(x[0].shift, false);
        assert_eq!(x[0].control, false);
        assert_eq!(x[0].meta, false);

        let y = tokenize_key_sequence("A").unwrap();
        assert_eq!(y.len(), 1);
        assert_eq!(y[0].key, "a");
        assert_eq!(y[0].shift, true);
        assert_eq!(y[0].control, false);
        assert_eq!(y[0].meta, false);

        let z = tokenize_key_sequence("$").unwrap();
        assert_eq!(z.len(), 1);
        assert_eq!(z[0].key, "$");
        assert_eq!(z[0].shift, false);
        assert_eq!(z[0].control, false);
        assert_eq!(z[0].meta, false);
    }
}
