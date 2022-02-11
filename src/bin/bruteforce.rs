use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        eprintln!("Usage: {} MAX_CHARS", args[0]);
        process::exit(1);
    }

    const BASE: u32 = 26;
    let chars: u32 = args[1].parse().unwrap();
    let end: u32 = BASE.checked_pow(chars).unwrap();

    for n in 0..end {
        let mut digits: Vec<u8> = Vec::new();

        let mut x = n;
        while x > 0 {
            let d: u32 = (x % BASE) + 65;
            digits.push(d.try_into().unwrap());
            x /= BASE
        }

        digits.reverse();

        let s: String = unsafe { String::from_utf8_unchecked(digits) };
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate() {
        // assert_eq!(reverse("SSKCYMDG").unwrap(), "PASSWORD".to_string());
    }
}
