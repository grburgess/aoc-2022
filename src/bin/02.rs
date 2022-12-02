use std::collections::HashMap;

use itertools::Itertools;

pub fn create_elf_hash() -> HashMap<String, u32> {
    let mut elf_map = HashMap::new();

    elf_map.insert("A".to_string(), 1);
    elf_map.insert("B".to_string(), 2);
    elf_map.insert("C".to_string(), 3);

    return elf_map;
}

pub fn create_me_hash(x: u32, y: u32, z: u32) -> HashMap<String, u32> {
    let mut elf_map = HashMap::new();

    elf_map.insert("X".to_string(), x);
    elf_map.insert("Y".to_string(), y);
    elf_map.insert("Z".to_string(), z);

    return elf_map;
}

struct Record {
    elf: String,
    me: String,
}

// fn parse_data(input: &str, output: &Vec<Record>) {
//     //let mut output: Vec<Record> = Vec::<Record>::new();

//     for line in input.lines() {
//         if !line.is_empty() {
//             let data: Vec<&str> = line.split(" ").collect();

//             output.push(Record {
//                 elf: data[0],
//                 me: data[1],
//             });
//         }
//     }

//     //return output;
// }

fn play_game(play: &Record, elf_map: &HashMap<String, u32>, me_map: &HashMap<String, u32>) -> u32 {
    let elf: u32 = elf_map.get(&play.elf).copied().unwrap();
    let me: u32 = me_map.get(&play.me).copied().unwrap();

    if elf == me {
        3 + me
    } else if elf > me {
        if me == 1 && elf == 3 {
            6 + me
        } else {
            me
        }
    } else {
        if me == 3 && elf == 1 {
            me
        } else {
            me + 6
        }
    }
}

fn play_game2(play: &Record, elf_map: &HashMap<String, u32>) -> u32 {
    let elf: u32 = elf_map.get(&play.elf).copied().unwrap();

    if play.me == "X" {
        if elf == 1 {
            3
        } else {
            elf - 1
        }
    } else if play.me == "Y" {
        3 + elf
    } else {
        if elf == 3 {
            7
        } else {
            elf + 1 + 6
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut plays: Vec<Record> = Vec::<Record>::new();

    let _my_input = String::from(input);

    for line in _my_input.lines() {
        if !line.is_empty() {
            let data: Vec<&str> = line.split(" ").collect();

            plays.push(Record {
                elf: data[0].to_string(),
                me: data[1].to_string(),
            });
        }
    }

    let elf_map = create_elf_hash();

    let mut xyz = vec![1, 2, 3];

    let me_map = create_me_hash(xyz[0], xyz[1], xyz[2]);

    let mut score = 0;

    for p in &plays {
        score += play_game(&p, &elf_map, &me_map);
    }

    Some(score)

    //    all_scores.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut plays: Vec<Record> = Vec::<Record>::new();

    let _my_input = String::from(input);

    for line in _my_input.lines() {
        if !line.is_empty() {
            let data: Vec<&str> = line.split(" ").collect();

            plays.push(Record {
                elf: data[0].to_string(),
                me: data[1].to_string(),
            });
        }
    }

    let elf_map = create_elf_hash();

    let mut score = 0;

    for p in &plays {
        score += play_game2(&p, &elf_map);
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
