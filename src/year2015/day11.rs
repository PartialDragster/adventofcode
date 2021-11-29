use std::str;

fn contains_increasing_straight(password: &str) -> bool {
    let password = password.as_bytes();
    for i in 1..password.len() - 2 {
        if password[i] == password[i+1] - 1 && password[i] == password[i+2] - 2 {
            return true;
        }
    }
    return false;
}

fn doesnt_contain_forbidden_character(password: &str) -> bool {
    return !password.contains("i")
        && !password.contains("o")
        && !password.contains("l");
}

fn contains_two_different_nonoverlapping_pairs(password: &str) -> bool {
    match password.chars()
        .try_fold( (None, None), |prev, current_character| 
                   {
                       match prev {
                           (Some(ch), Some(m)) => if ch == current_character && ch != m { Err("") } else { Ok ((Some(current_character), Some(m))) },
                           (Some(ch), None) => if ch == current_character { Ok((Some(current_character), Some(ch))) } else { Ok((Some(current_character), None)) },
                           (None, Some(_)) => panic!(), /* should never happen */
                           (None, None) => Ok((Some(current_character), None)),
                       }
                   }) {
            Ok(_) => false,
            Err(_) => true,
        }
}

fn increment_password(password: &mut str) -> String {
    unsafe {
        let password = password.as_bytes_mut();
        for i in (0..password.len()).rev() {
            if password[i] != 'z' as u8 {
                password[i] += 1;
                // hacky but lets us avoid chunks of the search space
                if password[i] == 'i' as u8 || password[i] == 'o' as u8 || password[i] == 'l' as u8 {
                    for j in i+1..password.len() {
                        password[j] = 'z' as u8;
                    }
                    return increment_password(&mut str::from_utf8(password).unwrap().to_string());
                } else {
                    return str::from_utf8(password).unwrap().to_string();
                }
            } else {
                password[i] = 'a' as u8;
            }
        }
    }
    panic!(); // should never get here
}

fn get_next_password(password: &str) -> String {
    let mut password = password.to_string();
    return loop {
        let new_password = increment_password(&mut password);
        if contains_increasing_straight(&new_password)
            && doesnt_contain_forbidden_character(&new_password)
            && contains_two_different_nonoverlapping_pairs(&new_password) {
                break new_password.to_string();
            }
        password = new_password;
    }
}

pub fn run() {
    let password = "hxbxwxba";
    let first_password = get_next_password(password);
    println!("{}", first_password);
    let second_password = get_next_password(&first_password);
    println!("{}", second_password);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn year2015_day11_test_passes_given_examples() {
        let password = "hijklmmn";
        assert_eq!(contains_increasing_straight(password), true);
        assert_eq!(doesnt_contain_forbidden_character(password), false);
        let password = "abbceffg";
        assert_eq!(contains_two_different_nonoverlapping_pairs(password), true);
        assert_eq!(contains_increasing_straight(password), false);
        let password = "abbcegjk";
        assert_eq!(contains_two_different_nonoverlapping_pairs(password), false);
        let password = "abcdefgh";
        assert_eq!(get_next_password(password), "abcdffaa");
        let password = "ghijklmn";
        assert_eq!(get_next_password(password), "ghjaabcc");
    }
}
