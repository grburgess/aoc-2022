use itertools::Itertools;
use std::cmp::Reverse;

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

pub fn part_one(input: &Input) -> Option<u32> {
    input.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let split = input.split("\n\n");

    let mut max1: u32 = 0;
    let mut max2: u32 = 0;
    let mut max3: u32 = 0;

    for s in split {
        //println!("{}", s);

        let sum: u32 = if s.ends_with("\n") {
            s[..s.len() - 1]
                .split("\n")
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        } else {
            s.split("\n")
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        };

        if sum > max1 {
            max3 = max2;
            max2 = max1;
            max1 = sum;
        } else if sum > max2 {
            max3 = max2;
            max2 = sum;
        } else if sum > max3 {
            max3 = sum;
        }
    }

    Some(max1 + max2 + max3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    let parsed = parse(&input);
    //part_one(input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    //  #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 1);
    //     assert_eq!(part_two(&input), None);
    // }
}
