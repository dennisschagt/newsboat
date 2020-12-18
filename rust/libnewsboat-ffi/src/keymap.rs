#[cxx::bridge(namespace = "newsboat::keymap::bridged")]
mod ffi {
    extern "Rust" {
        // `tokenize_operation_sequence()` returns `Option<Vec<Vec<String>>>`, but cxx doesn't
        // support `Option` and doesn't allow `Vec<Vec<_>>`. Here's how we work around that:
        //
        // 1. C++ doesn't care if the parse failed or just returned no operations, so we drop the
        //    `Option` and represent both `None` and `Some([])` with an empty vector (see
        //    `tokenize_operation_sequence()` code further down below);
        //
        // 2. we put `Vec<String>` into an opaque type `Operation`, and return `Vec<Operation>`.
        //    The opaque type is, in fact, a struct holding the `Vec<String>`. C++ extracts the
        //    vector using a helper function `operation_tokens()`.
        //
        // This is not very elegant, but doing the same by hand using `extern "C"` is prohibitively
        // complex.
        type Operation;
        fn tokenize_operation_sequence(input: &str, leftovers: &mut String) -> Vec<Operation>;
        fn operation_tokens(operation: &Operation) -> &Vec<String>;

        fn tokenize_operation_description(input: &str) -> String;

        type Key;
        fn tokenize_key_sequence(input: &str) -> Vec<Key>;
        fn get_key(key: &Key) -> String;
        fn get_shift(key: &Key) -> bool;
        fn get_control(key: &Key) -> bool;
        fn get_meta(key: &Key) -> bool;
    }

    extern "C++" {
        // cxx uses `std::out_of_range`, but doesn't include the header that defines that
        // exception. So we do it for them.
        include!("stdexcept");
        // Also inject a header that defines ptrdiff_t. Note this is *not* a C++ header, because
        // cxx uses a non-C++ name of the type.
        include!("stddef.h");
    }
}

struct Operation {
    tokens: Vec<String>,
}

struct Key {
    key: String,
    shift: bool,
    control: bool,
    meta: bool,
}

fn tokenize_operation_sequence(input: &str, leftovers: &mut String) -> Vec<Operation> {
    match libnewsboat::keymap::tokenize_operation_sequence(input) {
        Some((operations, remainder)) => {
            *leftovers = remainder.to_string();
            operations
                .into_iter()
                .map(|tokens| Operation { tokens })
                .collect::<Vec<_>>()
        }
        None => vec![],
    }
}

fn operation_tokens(input: &Operation) -> &Vec<String> {
    &input.tokens
}

fn tokenize_operation_description(input: &str) -> String {
    match libnewsboat::keymap::tokenize_operation_description(input) {
        Some(description) => String::from(description),
        None => String::new(),
    }
}

fn tokenize_key_sequence(input: &str) -> Vec<Key> {
    match libnewsboat::keymap::tokenize_key_sequence(input) {
        Some(key_sequence) => key_sequence
            .into_iter()
            .map(|k| Key {
                key: k.key,
                shift: k.shift,
                control: k.control,
                meta: k.meta,
            })
            .collect(),
        None => vec![],
    }
}

fn get_key(key: &Key) -> String {
    key.key.to_owned()
}
fn get_shift(key: &Key) -> bool {
    key.shift
}

fn get_control(key: &Key) -> bool {
    key.control
}

fn get_meta(key: &Key) -> bool {
    key.meta
}
