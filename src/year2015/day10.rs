fn look_and_say_proxy(numbers: &Vec<u32>) -> Vec<u32> {
    let (stats, mut result) = numbers[1..].iter()
        .fold(((numbers[0], 1), vec![]), 
            |(stats, mut result), new_number| if new_number == &stats.0 { 
                ((stats.0, stats.1 + 1), result) 
            } else { 
                result.push(stats.1);
                result.push(stats.0);
                ((*new_number, 1), result) 
            });
    result.push(stats.1);
    result.push(stats.0);
    result
}

fn look_and_say(original: &str) -> String {
    look_and_say_proxy(&original
                       .chars()
                       .map(|n| n.to_digit(10).unwrap())
                       .collect())
        .into_iter()
        .map(|i| i.to_string())
        .collect()
}

fn look_and_say_repeat(original: &str, repeat: u32) -> String {
    let mut as_vec = original.to_string();
    for _ in 0..repeat {
        as_vec = look_and_say(&as_vec);
    }
    as_vec
}

pub fn run() {
    println!("{}", look_and_say_repeat("1113122113", 40).len());
    println!("{}", look_and_say_repeat("1113122113", 50).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day10_test_look_say() {
       assert_eq!(look_and_say("1"), "11");  
       assert_eq!(look_and_say("11"), "21");  
       assert_eq!(look_and_say("21"), "1211");  
       assert_eq!(look_and_say("1211"), "111221");  
       assert_eq!(look_and_say("111221"), "312211");  
    }
}
