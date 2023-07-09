use base64::{engine::general_purpose::STANDARD as base64_standard, Engine as _};
use std::io::Read;

const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

enum State {
    MaybeBase64Utf8(MaybeBase64),
    InText,
}

struct MaybeBase64 {
    bytes: Vec<u8>,
    strikes: usize,
}

impl MaybeBase64 {
    fn new(byte: u8) -> Self {
        Self {
            bytes: vec![byte],
            strikes: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    fn decoded(&self) -> Option<String> {
        String::from_utf8(base64_standard.decode(&self.bytes.to_vec()).ok()?).ok()
    }

    fn raw(&self) -> String {
        self.bytes.iter().map(|b| *b as char).collect::<String>()
    }
}

fn main() {
    let mut state = State::InText;

    for byte in std::io::stdin().bytes().map(|b| b.unwrap()) {
        // println!(
        //     "{:?} state {}",
        //     byte as char,
        //     match state {
        //         State::InText => "InText".to_string(),
        //         State::MaybeBase64Utf8(ref maybe) =>
        //             format!("MaybeBase64Utf8({})", maybe.strikes).to_owned(),
        //     }
        // );
        match state {
            State::InText => {
                if BASE64_CHARS.contains(byte as char) {
                    state = State::MaybeBase64Utf8(MaybeBase64::new(byte));
                } else {
                    print!("{}", byte as char);
                }
            }
            State::MaybeBase64Utf8(ref mut maybe_base64) => {
                if BASE64_CHARS.contains(byte as char) {
                    maybe_base64.push(byte);
                    let decoded = maybe_base64.decoded();
                    if decoded.is_none() {
                        maybe_base64.strikes += 1;
                        if maybe_base64.strikes > 5 {
                            print!("{}", maybe_base64.raw());
                            state = State::InText;
                        }
                    } else {
                        maybe_base64.strikes = 0;
                    }
                } else {
                    if maybe_base64.strikes > 0 {
                        print!("{}", maybe_base64.raw());
                    } else {
                        let decoded = maybe_base64.decoded();
                        if let Some(decoded) = decoded {
                            println!("");
                            println!("===== decoded =====");
                            println!("{}", decoded);
                            println!("===================");
                        } else {
                            print!("{}", maybe_base64.raw());
                        }
                    }
                    print!("{}", byte as char);
                    state = State::InText;
                }
            }
        }
    }
}
