use std::{env, error::Error, process, result::Result};

fn reverse(hash: &str) -> Result<String, Box<dyn Error>> {
    if hash.is_empty() {
        return Ok("".to_string());
    }

    let mut as_nums: Vec<i32> = hash.as_bytes().iter().map(|c| (c - 65) as i32).collect();

    for i in (1..as_nums.len()).rev() {
        as_nums[i] = (as_nums[i] - as_nums[i - 1]).rem_euclid(26);
    }

    as_nums[0] -= 3;
    Ok(String::from_utf8(
        as_nums.iter().map(|c| (c + 65) as u8).collect(),
    )?)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        eprintln!("Usage: {} HASH", args[0]);
        process::exit(1);
    }

    let result = reverse(&args[1]).expect("Failed to parse hash");
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate() {
        assert_eq!(reverse("SSKCYMDG").unwrap(), "PASSWORD".to_string());
        assert_eq!(reverse("VZBVMUNL").unwrap(), "SECURITY".to_string());
        assert_eq!(
            reverse("DUYYJUSDREKZZRJFTKNJRKRDDQOQXXOOQJNEW").unwrap(),
            "AREALLYLONGPASSWORDWITHMANYCHARACTERS".to_string()
        );
    }
}
