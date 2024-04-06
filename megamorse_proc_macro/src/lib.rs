extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn morse(item: TokenStream) -> TokenStream {
    let as_string = item.to_string();

    let mut codes: Vec<&str> = Vec::with_capacity(as_string.len());

    for c in as_string.chars() {
        match c {
            ' ' => {}
            '.' => codes.push("MorseCode::Dot"),
            '-' => codes.push("MorseCode::Dash"),
            '_' => codes.push("MorseCode::Dash"),
            _ => panic!("Invalid character in morse code: {}", c),
        }
    }
    let joined = codes.join(", ");
    let full = format!("[{}]", joined);

    full.parse().unwrap()
}
