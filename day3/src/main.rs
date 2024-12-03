use aoclib::AocData;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Mul((isize, isize)),
    Do,
    Dont,
}

fn main() {
    let aoc = AocData::new("input.txt").unwrap();
    let inst_re: Regex =
        Regex::new(r"(do\(\)|don't\(\)|(mul\((\d+),(\d+)\)))").expect("a valid regex");

    let instructions: Vec<_> = inst_re
        .captures_iter(&aoc.input)
        .map(|cap| {
            let cap0 = cap.get(0).unwrap();
            match cap0.as_str() {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Dont,
                _ => {
                    let a = cap.get(3).unwrap().as_str().parse::<isize>().unwrap();
                    let b = cap.get(4).unwrap().as_str().parse::<isize>().unwrap();
                    Instruction::Mul((a, b))
                }
            }
        })
        .collect();

    let part1: isize = instructions
        .iter()
        .map(|i| {
            if let Instruction::Mul((a, b)) = i {
                a * b
            } else {
                0
            }
        })
        .sum();

    let mut part2 = 0;

    let mut enabled = true;

    for inst in instructions {
        match inst {
            Instruction::Mul((a, b)) => {
                if enabled {
                    part2 += a * b
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }

    dbg!(part1, part2);
}
