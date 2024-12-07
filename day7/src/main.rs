use aoclib::AocData;
use itertools::{all, Itertools};

struct Equation {
    test_val: usize,
    coefficients: Vec<usize>,
}

fn parse_line<'a>(mut line: impl Iterator<Item = &'a str>) -> Equation {
    let test_val = line.next().unwrap().replace(":", "").parse().unwrap();
    let coefficients: Vec<_> = line.map(|c| c.parse().unwrap()).collect();
    Equation {
        test_val,
        coefficients,
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn gen_ops(n: usize, allowed_ops: &[Op]) -> Vec<Vec<Op>> {
    (1..=n)
        .map(|_| Vec::from(allowed_ops))
        .multi_cartesian_product()
        .collect()
    // allowed_ops
    //     .iter()
    //     .copied()
    //     .combinations_with_replacement(n)
    //     .collect()
    // std::iter::repeat(allowed_ops)
    //     .take(n)
    //     .multi_cartesian_product()
    //     .map(|ops| ops.iter().map(|op| **op).collect())
    //     .collect()
}

fn apply_equation(eq: &Equation, mut ops: Vec<Op>) -> usize {
    let mut val = eq.coefficients[0];

    for c in &eq.coefficients[1..] {
        // Last bit of ops is our operator - 0 is add 1 is mul
        match ops.pop().unwrap() {
            Op::Add => val += c,
            Op::Mul => val *= c,
            Op::Concat => {
                let r_digits = (*c as f64).log10() as u32 + 1;
                let nval = (val * (10usize.pow(r_digits))) + c;
                val = nval;
            }
        }
    }

    val
}

fn coefficients_valid(eq: &Equation, allowed_ops: &Vec<Op>) -> bool {
    for ops in gen_ops(eq.coefficients.len() - 1, allowed_ops) {
        let val = apply_equation(eq, ops);
        if val == eq.test_val {
            return true;
        }
    }

    false
}

fn main() {
    let a = AocData::new("input.txt").unwrap();
    let equations: Vec<_> = a.rows().unwrap().map(parse_line).collect();

    let part1_ops = vec![Op::Add, Op::Mul];

    let part1: usize = equations
        .iter()
        .filter(|eq| coefficients_valid(eq, &part1_ops))
        .map(|eq| eq.test_val)
        .sum();

    eprintln!("===========================================");

    let part2_ops = vec![Op::Add, Op::Mul, Op::Concat];

    let part2: usize = equations
        .iter()
        .filter(|eq| coefficients_valid(eq, &part2_ops))
        .map(|eq| eq.test_val)
        .sum();

    dbg!(part1, part2);

    dbg!(gen_ops(2, &part2_ops));
}
