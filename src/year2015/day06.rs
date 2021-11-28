extern crate regex;

use regex::Regex;

use crate::year2015::utils;

#[derive(Debug)]
enum LightAction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: LightAction,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize
}

impl Instruction {
    fn new(action: LightAction, lower_left_coords: (usize, usize), upper_right_coords: (usize, usize)) -> Instruction {
        assert!(lower_left_coords.0 <= upper_right_coords.0);
        assert!(lower_left_coords.1 <= upper_right_coords.1);
        Instruction {
            action,
            x_min: lower_left_coords.0,
            x_max: upper_right_coords.0,
            y_min: lower_left_coords.1,
            y_max: upper_right_coords.1,
        }
    }
}

fn parse_line(line: &str) -> Instruction {
    let regex = Regex::new(r"(turn on|turn off|toggle) (.*),(.*) through (.*),(.*)").unwrap();
    let caps = regex.captures(line).unwrap();

    let lower_left_coords: (usize, usize) 
        = (caps[2].parse().unwrap(), caps[3].parse().unwrap());
    let upper_right_coords: (usize, usize) 
        = (caps[4].parse().unwrap(), caps[5].parse().unwrap());

    match &caps[1] {
        "turn on" => Instruction::new(LightAction::TurnOn, lower_left_coords, upper_right_coords),
        "turn off" => Instruction::new(LightAction::TurnOff, lower_left_coords, upper_right_coords),
        "toggle" => Instruction::new(LightAction::Toggle, lower_left_coords, upper_right_coords),
        _ => panic!()
    }
}

fn take_action(instruction: &Instruction, x: u32) -> u32 {
    match instruction.action {
        LightAction::TurnOn =>  1,
        LightAction:: TurnOff =>  0,
        LightAction::Toggle => 1-x,
    }
}

fn new_take_action(instruction: &Instruction, x: u32) -> u32 {
    match instruction.action {
        LightAction::TurnOn => x + 1,
        LightAction::TurnOff => if x <= 0 { 0 } else { x - 1}
        LightAction::Toggle => x + 2,
    }
}

fn apply_instructions(is_light_on: &mut Vec<Vec<u32>>, instruction_set: &Vec<Instruction>, action: &dyn Fn(&Instruction, u32)->u32) {
    for instruction in instruction_set {
        for i in instruction.x_min..(instruction.x_max+1) {
            for j in instruction.y_min..(instruction.y_max+1) {
                is_light_on[i][j] = action(instruction, is_light_on[i][j]);
            }
        }
    };
}

pub fn run() {
    let mut is_light_on = vec![vec![0; 1000]; 1000];
    let input = utils::read_file_to_lines("data/year2015/day06");
    let instruction_set = input.iter().map(|line| parse_line(line)).collect();
    apply_instructions(&mut is_light_on, &instruction_set, &take_action);
    let lights_on = is_light_on.iter().fold(0, |a1, v1| a1 + v1.iter().fold(0, |a2, v2| a2 + v2));
    println!("{:?}", lights_on);
    let mut is_light_on = vec![vec![0; 1000]; 1000];
    apply_instructions(&mut is_light_on, &instruction_set, &new_take_action);
    let lights_on = is_light_on.iter().fold(0, |a1, v1| a1 + v1.iter().fold(0, |a2, v2| a2 + v2));
    println!("{:?}", lights_on);
}
