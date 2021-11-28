use md5;

fn work_proof(key: &str, zeroes: usize) -> Option<u32> {
    for x in 1.. {
        let ext_key = format!("{}{}", key, x);
        let digest: String = format!("{:x}", md5::compute(ext_key));
        if digest.starts_with(&"0".repeat(zeroes)) {
            return Some(x);
        }
    }
    None
}

pub fn run() {
    let key = "bgvyzdsv";
    println!("{}", work_proof(key, 5).unwrap());
    println!("{}", work_proof(key, 6).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day04_test_work_proof() {
        assert_eq!(work_proof("abcdef", 5).unwrap(), 609043);
        assert_eq!(work_proof("pqrstuv", 5).unwrap(), 1048970);
    }
}
