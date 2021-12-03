extern crate regex;

use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::utils::utils;

struct U16ary {
    value: u16,
    instruction: String,
}

impl U16ary {
    fn new(value: u16, instruction: String) -> U16ary {
        U16ary {
            value,
            instruction,
        }
    }
}

trait Runnable {
    fn run(&mut self) -> u16;
    fn get_instruction(&self) -> &String;
    fn reset(&mut self);
    fn set_signal(&mut self, value: u16);
}

impl Runnable for U16ary {
    fn run(&mut self) -> u16 {
        self.value
    }

    fn get_instruction(&self) -> &String {
        &self.instruction
    }

    fn reset(&mut self) { }

    fn set_signal(&mut self, _value: u16) {
        panic!();
    }
}

struct Zeroary { 
    value: Rc<RefCell<Box<dyn Runnable>>>,
    instruction: String,
    calculated_value: Option<u16>,
}

impl Zeroary {
    fn new(value: Rc<RefCell<Box<dyn Runnable>>>, instruction: String) -> Zeroary {
        Zeroary {
            instruction,
            value,
            calculated_value: None,
        }
    }
}

impl Runnable for Zeroary {
    fn run(&mut self) -> u16 {
        match self.calculated_value {
            None => {
                let x = self.value.borrow_mut().run();
                self.calculated_value = Some(x);
                x
            },
            Some(x) => x,
        }
    }

    fn get_instruction(&self) -> &String {
        &self.instruction
    }

    fn reset(&mut self) {
        self.calculated_value = None;
    }

    fn set_signal(&mut self, value: u16) {
        self.calculated_value = Some(value);
    }
}

enum UnaryOperator {
    Not,
}

struct Unary {
    operator: UnaryOperator,
    argument: Rc<RefCell<Box<dyn Runnable>>>,
    instruction: String,
    calculated_value: Option<u16>,
}

impl Unary {
    fn new(operator: UnaryOperator, argument: Rc<RefCell<Box<dyn Runnable>>>, instruction: String) -> Unary {
        Unary {
            operator,
            argument,
            instruction,
            calculated_value: None,
        }
    }
}

impl Runnable for Unary {
    fn run(&mut self) -> u16 {
        match self.calculated_value {
            None => {
                let op: fn(u16) -> u16 = match self.operator {
                    UnaryOperator::Not => |x| !x
                };
                let x = op(self.argument.borrow_mut().run());
                self.calculated_value = Some(x);
                x
            },
            Some(x) => x,
        }
    }

    fn get_instruction(&self) -> &String {
        &self.instruction
    }

    fn reset(&mut self) {
        self.calculated_value = None;
    }

    fn set_signal(&mut self, value: u16) {
        self.calculated_value = Some(value);
    }
}

enum BinaryOperator {
    And,
    LShift,
    Or,
    RShift,
}

struct Binary {
    operator: BinaryOperator,
    left_argument: Rc<RefCell<Box<dyn Runnable>>>,
    right_argument: Rc<RefCell<Box<dyn Runnable>>>,
    instruction: String,
    calculated_value: Option<u16>,
}

impl Binary {
    fn new( left_argument: Rc<RefCell<Box<dyn Runnable>>>, 
            operator: BinaryOperator, 
            right_argument: Rc<RefCell<Box<dyn Runnable>>>,
            instruction: String,) -> Binary {
        Binary {
            operator,
            left_argument,
            right_argument,
            instruction,
            calculated_value: None,
        }
    }
}

impl Runnable for Binary {
    fn run(&mut self) -> u16 {
        match self.calculated_value {
            None => {
                let op: fn(u16,u16) -> u16 = match self.operator {
                    BinaryOperator::And => |x,y| x & y,
                    BinaryOperator::LShift => |x,y| x << y,
                    BinaryOperator::Or => |x,y| x | y,
                    BinaryOperator::RShift => |x,y| x >> y,
                };
                let larg = self.left_argument.borrow_mut().run();
                let rarg = self.right_argument.borrow_mut().run();
                let x = op(larg, rarg);
                self.calculated_value = Some(x);
                x
            },
            Some(x) => x
        }
    }

    fn get_instruction(&self) -> &String {
        &self.instruction
    }

    fn reset(&mut self) {
        self.calculated_value = None;
    }

    fn set_signal(&mut self, value: u16) {
        self.calculated_value = Some(value);
    }
}

fn parse(line: &str, map: &mut HashMap<String, Vec<String>>) {
    let re = Regex::new(r"(.*) (.*) (.*) -> (.*)").unwrap();
    let caps = re.captures(line);
    if let Some(caps) = caps {
        map.insert(caps[4].to_string(), (1..4).map(|n| caps[n].to_string()).collect());
        return;
    }

    let re = Regex::new(r"(.*) (.*) -> (.*)").unwrap();
    let caps = re.captures(line);
    if let Some(caps) = caps {
        map.insert(caps[3].to_string(), (1..3).map(|n| caps[n].to_string()).collect());
        return;
    }

    let re = Regex::new(r"(.*) -> (.*)").unwrap();
    let caps = re.captures(line);
    if let Some(caps) = caps {
        map.insert(caps[2].to_string(), vec![caps[1].to_string()]);
        return;
    }
}

fn follow_key<'a>(key: &'a str, instruction_strings: &'a HashMap<String, Vec<String>>, found_strings: &'a HashMap<String, Rc<RefCell<Box<dyn Runnable>>>>) -> &'a str {
    let instruction = &instruction_strings[key];
    match instruction.len() {
        3 => if !instruction[0].chars().all(char::is_numeric) && !found_strings.contains_key(&instruction[0]) {
                follow_key(&instruction[0], instruction_strings, found_strings)
            } else if !instruction[2].chars().all(char::is_numeric) && !found_strings.contains_key(&instruction[2]) {
                follow_key(&instruction[2], instruction_strings, found_strings) 
            } else {
                key
            },
        2 => if !instruction[1].chars().all(char::is_numeric) && !found_strings.contains_key(&instruction[1]) { 
                follow_key(&instruction[1], instruction_strings, found_strings) 
            } else { 
                key 
            },
        1 => if !instruction[0].chars().all(char::is_numeric) && !found_strings.contains_key(&instruction[0]) { 
            follow_key(&instruction[0], instruction_strings, found_strings) 
        } else { 
            key 
        },
        _ => {
            eprintln!("Couldn't understand {:?} for key {}", instruction, key);
            panic!();
        },
    }
}

fn find_leaf_key<'a>(instruction_strings: &'a HashMap<String, Vec<String>>, found_strings: &'a HashMap<String, Rc<RefCell<Box<dyn Runnable>>>>) -> Option<&'a str> {
    if instruction_strings.len() == found_strings.len() { return None; }

    let init_key = instruction_strings.keys().filter(|k| !found_strings.contains_key(&k[..])).next().unwrap();
    Some(follow_key(&init_key, &instruction_strings, &found_strings))
}

fn build_network(instruction_strings: &HashMap<String, Vec<String>>) -> HashMap<String, Rc<RefCell<Box<dyn Runnable>>>> {
    let mut network_map = HashMap::new();
    while let Some(key) = find_leaf_key(&instruction_strings, &network_map) {
        let key = key.to_string();
        let instruction = &instruction_strings[&key];
        let node: Rc<RefCell<Box<dyn Runnable>>> = match instruction.len() {
            3 => {
                let larg: Rc<RefCell<Box<dyn Runnable>>> = if instruction[0].chars().all(char::is_numeric) { 
                    Rc::new(RefCell::new(Box::new(U16ary::new(instruction[0].parse::<u16>().unwrap(),instruction.join(" ")))))
                } else { 
                    network_map[&instruction[0]].clone()
                };
                let op = match &instruction[1][..] {
                    "AND" => BinaryOperator::And,
                    "LSHIFT" => BinaryOperator::LShift,
                    "OR" => BinaryOperator::Or,
                    "RSHIFT" => BinaryOperator::RShift,
                    o => {
                        eprintln!("Operator {} not recognised", o);
                        panic!();
                    }
                };
                let rarg: Rc<RefCell<Box<dyn Runnable>>> = if instruction[2].chars().all(char::is_numeric) { 
                    Rc::new(RefCell::new(Box::new(U16ary::new(instruction[2].parse::<u16>().unwrap(),instruction.join(" ")))))
                } else { 
                    network_map[&instruction[2]].clone()
                };
                Rc::new(RefCell::new(Box::new(Binary::new(larg, op, rarg, instruction.join(" ")))))
            }
            2 => { 
                let arg: Rc<RefCell<Box<dyn Runnable>>> = if instruction[1].chars().all(char::is_numeric) { 
                    Rc::new(RefCell::new(Box::new(U16ary::new(instruction[1].parse::<u16>().unwrap(),instruction.join(" ")))))
                } else { 
                    network_map[&instruction[1]].clone()
                };
                let op = match &instruction[0][..] {
                    "NOT" => UnaryOperator::Not,
                    o => {
                        eprintln!("Operator {} not recognised", o);
                        panic!();
                    }
                };
                Rc::new(RefCell::new(Box::new(Unary::new(op, arg, instruction.join(" ")))))
            },
            1 => {
                let arg: Rc<RefCell<Box<dyn Runnable>>> = if instruction[0].chars().all(char::is_numeric) { 
                    Rc::new(RefCell::new(Box::new(U16ary::new(instruction[0].parse::<u16>().unwrap(),instruction.join(" ")))))
                } else { 
                    network_map[&instruction[0]].clone()
                };
                Rc::new(RefCell::new(Box::new(Zeroary::new(arg, instruction.join(" ")))))
            },
            _ => {
                eprintln!("Couldn't understand {:?} for key {}", instruction, key);
                panic!();
            }
        };
        network_map.insert(key.to_string(), node);
    }

    network_map
}

pub fn run() {
    let input = utils::read_file_to_lines("data/year2015/day07");
    let mut map = HashMap::new();
    input.iter()
        .map(|line| parse(&line, &mut map))
        .for_each(drop);
    let network = build_network(&map);
    let provided_a = network["a"].borrow_mut().run();
    println!("{:?}", provided_a);
    for node in network.values() {
        node.borrow_mut().reset();
    }
    network["b"].borrow_mut().set_signal(provided_a);
    println!("{:?}", network["a"].borrow_mut().run());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day07_test_sample_circuit() {
        let example = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        let mut map = HashMap::new();
        example.iter()
            .map(|line| parse(&line, &mut map))
            .for_each(drop);
        let network = build_network(&map);
        assert_eq!(network["d"].borrow_mut().run(), 72);
        assert_eq!(network["e"].borrow_mut().run(), 507);
        assert_eq!(network["f"].borrow_mut().run(), 492);
        assert_eq!(network["g"].borrow_mut().run(), 114);
        assert_eq!(network["h"].borrow_mut().run(), 65412);
        assert_eq!(network["i"].borrow_mut().run(), 65079);
        assert_eq!(network["x"].borrow_mut().run(), 123);
        assert_eq!(network["y"].borrow_mut().run(), 456);
    }
}
