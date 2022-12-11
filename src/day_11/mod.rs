#[path = "../utils/mod.rs"] mod utils;

#[derive(Copy, Clone, Default)]
enum Operation { Sum(u64), Mul(u64), #[default] Sq }

impl Operation {
    fn calculate(self, value: u64) -> u64 {
        match self {
            Operation::Sum(x) => value + x,
            Operation::Mul(x) => value * x,
            Operation::Sq => value * value,
        }
    }
}

#[derive(Clone, Default)]
pub struct Monkey {
    items: std::collections::VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: u32,
    if_false: u32,
}

fn parse_lines(lines: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for line in lines {
        let split = line.split(' ').collect::<Vec<&str>>();
        if split.len() == 1 { continue; }
         
        if split[0] == "Monkey" {
            monkeys.push(Monkey::default());
            continue;
        } 
        let last_index = monkeys.len() - 1;
        if split[2] == "Starting" {
            monkeys[last_index].items.extend(line[18..].split(',') .map(|x| x.trim()).map(|x| x.parse::<u64>().unwrap()).clone());
        } else if split[2] == "Operation:" {
            if split[6] == "+" {
                monkeys[last_index].operation = Operation::Sum(split[7].parse::<u64>().unwrap());
            } else {
                if split[7] == "old" {
                    monkeys[last_index].operation = Operation::Sq;
                } else {
                    monkeys[last_index].operation = Operation::Mul(split[7].parse::<u64>().unwrap());
                }
            }
        } else if split[2] == "Test:" {
            monkeys[last_index].test = split[5].parse::<u64>().unwrap();
        } else if split[4] == "If" {
            if split[5] == "true:" {
                monkeys[last_index].if_true = split[9].parse::<u32>().unwrap();
            } else {
                monkeys[last_index].if_false = split[9].parse::<u32>().unwrap();
            }
        }
    }
    return monkeys;
}

fn play_round(monkeys: &mut Vec<Monkey>, activity: &mut Vec<u64>, relief_divisor: u64) {
    for i in 0..monkeys.len() {
        activity[i] += monkeys[i].items.len() as u64;
        
        while let Some(item) = monkeys[i].items.pop_front() {
            let mut worry_level = monkeys[i].operation.calculate(item);
            worry_level %= monkeys.iter().map(|m| m.test).product::<u64>();
            worry_level /= relief_divisor;

            if worry_level % monkeys[i].test == 0 {
                let m = monkeys[i].if_true as usize;
                monkeys[m].items.push_back(worry_level);
            } else {
                let m = monkeys[i].if_false as usize;
                monkeys[m].items.push_back(worry_level);
            };
        }
    }
}

fn play_rounds(monkeys: &Vec<Monkey>, rounds: u32, relief_divisor: u64) -> u64 {
    let mut monkeys = monkeys.to_vec();
    let mut activity = vec![0; monkeys.len()];
    for _ in 0..rounds { play_round(&mut monkeys, &mut activity, relief_divisor); }
    activity.sort();
    activity[activity.len() - 2] * activity[activity.len() - 1]
}

pub fn run()
{
    let lines = utils::lines_from_file("./src/day_11/input.txt").expect("Failed to read line from file");
    let monkeys = parse_lines(&lines);
    let answer1 = play_rounds(&monkeys, 20, 3);
    let answer2 = play_rounds(&monkeys, 10000, 1);
    assert_eq!(answer1, 61005);
    assert_eq!(answer2, 20567144694);
}