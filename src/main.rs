use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead, BufReader, Error},
    path::PathBuf,
    process,
    string::FromUtf8Error,
};

fn hash(input: &str) -> Result<String, FromUtf8Error> {
    let stripped = input
        .to_ascii_uppercase()
        .replace(|c: char| !c.is_ascii_alphabetic(), "");

    if stripped.is_empty() {
        return Ok("".to_string());
    }

    let mut as_nums: Vec<u8> = stripped.into_bytes().iter().map(|c| c - 65).collect();

    as_nums[0] = (as_nums[0] + 3) % 26;

    for i in 1..as_nums.len() {
        as_nums[i] = (as_nums[i - 1] + as_nums[i]) % 26;
    }

    String::from_utf8(as_nums.iter().map(|c| (c + 65) as u8).collect())
}

fn load_leaked_passwords(filename: PathBuf) -> Result<HashMap<String, String>, Error> {
    let f = File::open(filename)?;
    let r = BufReader::new(f);
    let mut passwords: HashMap<String, String> = HashMap::new();

    for l in r.lines() {
        let l = l?;
        let s: Vec<&str> = l.trim().split(",").collect();
        passwords.insert(s[0].to_string(), s[1].to_string());
    }

    Ok(passwords)
}

fn load_dictionary(filename: PathBuf) -> Result<io::Lines<io::BufReader<File>>, Error> {
    let f = File::open(filename)?;
    Ok(io::BufReader::new(f).lines())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 3 {
        eprintln!("Usage: {} LEAKED_LIST DICTIONARY", args[0]);
        process::exit(1);
    }

    let mut passwords = load_leaked_passwords(PathBuf::from(&args[1]))?;
    let dictionary = load_dictionary(PathBuf::from(&args[2]))?;
    let mut progress = 0;

    for word in dictionary.map(Result::unwrap) {
        let mut solved: Vec<String> = Vec::new();

        if passwords.is_empty() {
            eprintln!("All done");
        }

        for (k, v) in passwords.iter() {
            if hash(&word).unwrap() == *v {
                println!("Hash {} for user {} is password '{}'", v, k, word);
                solved.push(k.to_string());
            }
        }

        for s in solved {
            passwords.remove(&s);
        }

        progress += 1;
        if progress % 1000 == 0 {
            eprintln!("Completed {} words with {} hashes remaining", progress, passwords.len());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate() {
        assert_eq!(hash("PASSWORD"), Ok("SSKCYMDG".to_string()));
        assert_eq!(hash("SECURITY"), Ok("VZBVMUNL".to_string()));
        assert_eq!(
            hash("AREALLYLONGPASSWORDWITHMANYCHARACTERS"),
            Ok("DUYYJUSDREKZZRJFTKNJRKRDDQOQXXOOQJNEW".to_string())
        );
    }
}
