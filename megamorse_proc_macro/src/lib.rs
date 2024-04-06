extern crate proc_macro;
use proc_macro::TokenStream;

macro_rules! err {
    ($e:expr) => {{
        let err = format!("compile_error!(\"{}\")", $e);

        return err.parse().unwrap();
    }};
}

#[proc_macro]
pub fn morse(item: TokenStream) -> TokenStream {
    let as_string = item.to_string();
    let mut parsed_words: Vec<String> = Vec::new();

    let words = as_string.split_whitespace();

    for word in words {
        if word.len() > 5 {
            err!(format!(
                "Word is too long: '{}'.\nMaximum length is 5, actual length is {}",
                word,
                word.len()
            ));
        }

        let mut codes: Vec<&str> = Vec::with_capacity(as_string.len());

        for c in word.chars() {
            match c {
                ' ' => {}
                '.' => codes.push("megamorse::MorseCode::Dot"),
                '-' => codes.push("megamorse::MorseCode::Dash"),
                '_' => codes.push("megamorse::MorseCode::Dash"),
                _ => err!(format!("Invalid character in morse code: {}", c)),
            }
        }
        let joined = codes.join(", ");
        let full = format!("megamorse::MorseWord::from([{}])", joined);

        parsed_words.push(full);
    }

    let joined = parsed_words.join(", ");
    let out_string = format!("[{}]", joined);

    out_string.parse().unwrap()
}
