use crate::utils::utils;
use reduce::Reduce;
use std::ops::Add;

lalrpop_mod!(pub snail_number, "/year2021/snail_number.rs");

#[derive(Debug, PartialEq, Clone)]
pub enum SnailNumber {
    Node(Box<SnailNumber>, Box<SnailNumber>),
    Leaf(u32),
}

#[derive(Debug)]
enum ExplosionResult {
    Explode(u32, u32),
    CarryToLeft(u32),
    CarryToRight(u32),
    DescendentExploded,
    None,
}

enum SplitResult {
    Split(u32, u32),
    DescendentSplit,
    None,
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = SnailNumber::Node(Box::new(self.clone()), Box::new(other.clone()));

        loop {
            let r = result.try_explode(0);
            match r {
                ExplosionResult::None => (),
                _ => continue,
            }

            match result.try_split() {
                SplitResult::None => (),
                _ => continue,
            }

            return result;
        }
    }
}

impl SnailNumber {
    fn magnitude(&self) -> u32 {
        match self {
            SnailNumber::Node(n1, n2) => 3*n1.magnitude() + 2*n2.magnitude(),
            SnailNumber::Leaf(v) => *v,
        }
    }

    fn split_once(mut self) -> Self {
        self.try_split();
        self
    }

    fn try_split(&mut self) -> SplitResult {
        match self {
            SnailNumber::Node(ref mut n1, ref mut n2) => {
                match n1.try_split() {
                    SplitResult::Split(v1, v2) => {
                        *n1 = Box::new(SnailNumber::Node(Box::new(SnailNumber::Leaf(v1)), Box::new(SnailNumber::Leaf(v2))));
                        return SplitResult::DescendentSplit;
                    },
                    SplitResult::DescendentSplit => {
                        return SplitResult::DescendentSplit;
                    },
                    SplitResult::None => (),
                }

                match n2.try_split() {
                    SplitResult::Split(v1, v2) => {
                        *n2 = Box::new(SnailNumber::Node(Box::new(SnailNumber::Leaf(v1)), Box::new(SnailNumber::Leaf(v2))));
                        return SplitResult::DescendentSplit;
                    },
                    SplitResult::DescendentSplit => {
                        return SplitResult::DescendentSplit;
                    },
                    SplitResult::None => (),
                }
            },
            SnailNumber::Leaf(v) => {
                if *v >= 10 {
                    return SplitResult::Split(*v / 2, *v / 2 + *v % 2);
                } else {
                    return SplitResult::None;
                }
            },
        }
        SplitResult::None
    }

    fn explode_once(mut self) -> Self {
        self.try_explode(0);
        self
    }

    fn try_explode(&mut self, depth: u32) -> ExplosionResult {
        if depth > 4 { panic!("depth greater than 4"); }
        if depth == 4 {
            match self {
                SnailNumber::Node(n1, n2) => {
                    if let SnailNumber::Leaf(v1) = **n1 {
                        if let SnailNumber::Leaf(v2) = **n2 {
                            return ExplosionResult::Explode(v1, v2)
                        }
                    }
                    panic!("exploding pair not cherry");
                },
                SnailNumber::Leaf(_) => {
                    return ExplosionResult::None;
                },
            }
        }

        match self {
            SnailNumber::Node(ref mut left_child, ref mut right_child) => {
                match left_child.try_explode(depth+1) {
                    ExplosionResult::Explode(v1, v2) => {
                        *left_child = Box::new(SnailNumber::Leaf(0));
                        right_child.carry_to_right(v2);
                        return ExplosionResult::CarryToLeft(v1);
                    },
                    ExplosionResult::CarryToLeft(v1) => {
                        return ExplosionResult::CarryToLeft(v1);
                    },
                    ExplosionResult::CarryToRight(v2) => {
                        right_child.carry_to_right(v2);
                        return ExplosionResult::DescendentExploded;
                    },
                    ExplosionResult::DescendentExploded => {
                        return ExplosionResult::DescendentExploded;
                    },
                    ExplosionResult::None => (),
                };

                match right_child.try_explode(depth+1) {
                    ExplosionResult::Explode(v1, v2) => {
                        *right_child = Box::new(SnailNumber::Leaf(0));
                        left_child.carry_to_left(v1);
                        return ExplosionResult::CarryToRight(v2);
                    },
                    ExplosionResult::CarryToLeft(v1) => {
                        left_child.carry_to_left(v1);
                        return ExplosionResult::DescendentExploded;
                    },
                    ExplosionResult::CarryToRight(v2) => {
                        return ExplosionResult::CarryToRight(v2);
                    },
                    ExplosionResult::DescendentExploded => {
                        return ExplosionResult::DescendentExploded;
                    },
                    ExplosionResult::None => (),
                };
            }
            SnailNumber::Leaf(_) => {
                return ExplosionResult::None;
            }
        };

        ExplosionResult::None
    }

    fn carry_to_left(&mut self, v1: u32) {
        match self {
            SnailNumber::Leaf(v) => *v += v1,
            SnailNumber::Node(_, n2) => n2.carry_to_left(v1),
        }
    }

    fn carry_to_right(&mut self, v2: u32) {
        match self {
            SnailNumber::Leaf(v) => *v += v2,
            SnailNumber::Node(n1, _) => n1.carry_to_right(v2),
        }
    }
}

pub fn run() {
    let magnitude = utils::read_file_to_lines("data/year2021/day18")
        .iter()
        .map(|s| snail_number::TermParser::new().parse(s).unwrap())
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude();
    println!("{}", magnitude);

    let numbers: Vec<SnailNumber> = utils::read_file_to_lines("data/year2021/day18")
        .iter()
        .map(|s| snail_number::TermParser::new().parse(s).unwrap())
        .collect();
    let mut largest_magnitude = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let magnitude = (numbers[i].clone() + numbers[j].clone()).magnitude();
                if magnitude > largest_magnitude {
                    largest_magnitude = magnitude;
                }
            }
        }
    }
    println!("{}", largest_magnitude);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day18() {
        assert!(snail_number::TermParser::new().parse(r"[1,2]").is_ok());
        assert!(snail_number::TermParser::new().parse(r"[[1,2],3]").is_ok());
        assert!(snail_number::TermParser::new().parse(r"[9,[8,7]]").is_ok());
        assert!(snail_number::TermParser::new().parse(r"[[1,9],[8,5]]").is_ok());
        assert!(snail_number::TermParser::new().parse(r"[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").is_ok());
        assert!(snail_number::TermParser::new().parse(r"[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]").is_ok());
        assert!(snail_number::TermParser::new().parse(r"[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").is_ok());
        assert_eq!(snail_number::TermParser::new().parse(r"[[[[[9,8],1],2],3],4]").unwrap().explode_once(), 
                   snail_number::TermParser::new().parse(r"[[[[0,9],2],3],4]").unwrap());
        assert_eq!(snail_number::TermParser::new().parse(r"[7,[6,[5,[4,[3,2]]]]]").unwrap().explode_once(), 
                   snail_number::TermParser::new().parse(r"[7,[6,[5,[7,0]]]]").unwrap());
        assert_eq!(snail_number::TermParser::new().parse(r"[[6,[5,[4,[3,2]]]],1]").unwrap().explode_once(), 
                   snail_number::TermParser::new().parse(r"[[6,[5,[7,0]]],3]").unwrap());
        assert_eq!(snail_number::TermParser::new().parse(r"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap().explode_once(), 
                   snail_number::TermParser::new().parse(r"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap());
        assert_eq!(snail_number::TermParser::new().parse(r"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap().explode_once(), 
                   snail_number::TermParser::new().parse(r"[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap());

        let n = snail_number::TermParser::new().parse(r"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        let n = n.explode_once();
        assert_eq!(&n, &snail_number::TermParser::new().parse(r"[[[[0,7],4],[7,[[8,4],9]]],[1,1]]").unwrap());
        let n = n.explode_once();
        assert_eq!(&n, &snail_number::TermParser::new().parse(r"[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap());
        let n = n.split_once();
        assert_eq!(&n, &snail_number::TermParser::new().parse(r"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap());
        let n = n.split_once();
        assert_eq!(&n, &snail_number::TermParser::new().parse(r"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap());
        let n = n.explode_once();
        assert_eq!(&n, &snail_number::TermParser::new().parse(r"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap());

        let a = snail_number::TermParser::new().parse(r"[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let b = snail_number::TermParser::new().parse(r"[1,1]").unwrap();
        assert_eq!(a + b, snail_number::TermParser::new().parse(r"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap());

        let a = snail_number::TermParser::new().parse(r"[1,1]").unwrap();
        let b = snail_number::TermParser::new().parse(r"[2,2]").unwrap();
        let c = snail_number::TermParser::new().parse(r"[3,3]").unwrap();
        let d = snail_number::TermParser::new().parse(r"[4,4]").unwrap();
        let s = snail_number::TermParser::new().parse(r"[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        assert_eq!(a + b + c + d, s);

        let a = snail_number::TermParser::new().parse(r"[1,1]").unwrap();
        let b = snail_number::TermParser::new().parse(r"[2,2]").unwrap();
        let c = snail_number::TermParser::new().parse(r"[3,3]").unwrap();
        let d = snail_number::TermParser::new().parse(r"[4,4]").unwrap();
        let e = snail_number::TermParser::new().parse(r"[5,5]").unwrap();
        let s = snail_number::TermParser::new().parse(r"[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        assert_eq!(a + b + c + d + e, s);

        let a = snail_number::TermParser::new().parse(r"[1,1]").unwrap();
        let b = snail_number::TermParser::new().parse(r"[2,2]").unwrap();
        let c = snail_number::TermParser::new().parse(r"[3,3]").unwrap();
        let d = snail_number::TermParser::new().parse(r"[4,4]").unwrap();
        let e = snail_number::TermParser::new().parse(r"[5,5]").unwrap();
        let f = snail_number::TermParser::new().parse(r"[6,6]").unwrap();
        let s = snail_number::TermParser::new().parse(r"[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(a + b + c + d + e + f, s);

        let a = snail_number::TermParser::new().parse(r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").unwrap();
        let b = snail_number::TermParser::new().parse(r"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap();
        let c = snail_number::TermParser::new().parse(r"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]").unwrap();
        let d = snail_number::TermParser::new().parse(r"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]").unwrap();
        let e = snail_number::TermParser::new().parse(r"[7,[5,[[3,8],[1,4]]]]").unwrap();
        let f = snail_number::TermParser::new().parse(r"[[2,[2,2]],[8,[8,1]]]").unwrap();
        let g = snail_number::TermParser::new().parse(r"[2,9]").unwrap();
        let h = snail_number::TermParser::new().parse(r"[1,[[[9,3],9],[[9,0],[0,7]]]]").unwrap();
        let i = snail_number::TermParser::new().parse(r"[[[5,[7,4]],7],1]").unwrap();
        let j = snail_number::TermParser::new().parse(r"[[[[4,2],2],6],[8,7]]
").unwrap();
        let s = snail_number::TermParser::new().parse(r"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(a + b + c + d + e + f + g + h + i + j, s);

        assert_eq!(snail_number::TermParser::new().parse(r"[[1,2],[[3,4],5]]").unwrap().magnitude(), 143);
        assert_eq!(snail_number::TermParser::new().parse(r"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().magnitude(), 1384);
        assert_eq!(snail_number::TermParser::new().parse(r"[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap().magnitude(), 445);
        assert_eq!(snail_number::TermParser::new().parse(r"[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().magnitude(), 791);
        assert_eq!(snail_number::TermParser::new().parse(r"[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().magnitude(), 1137);
        assert_eq!(snail_number::TermParser::new().parse(r"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap().magnitude(), 3488);
    }
}
