#[path = "../utils/mod.rs"] mod utils;

fn parse_stacks(lines: &Vec<String>, stack_count: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];
    for line in lines {
        if line.chars().collect::<Vec<char>>()[1] == '1'{
            break;
        }
        for i in 0..stack_count {
            let char = line.chars().collect::<Vec<char>>()[(4 * i) + 1];
            if char != ' '{
                stacks[i].push(char);
            }
        }
    }
    return stacks;

} 

fn parse_moves(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut moves = vec![];
    for line in lines {
        if !line.is_empty() && line.chars().collect::<Vec<char>>()[0] == 'm'  {
            let split = line.split(' ').collect::<Vec<&str>>();
            let mut current_move= vec![];
            current_move.push(split[1].parse::<u32>().unwrap());
            current_move.push(split[3].parse::<u32>().unwrap());
            current_move.push(split[5].parse::<u32>().unwrap());
            moves.push(current_move);
        }
    }
    return moves;
} 

fn pop_stack(stacks: &mut Vec<Vec<char>>, from_stack: u32) -> char {
    return stacks[from_stack as usize].remove(0);
} 

fn push_stack(stacks: &mut Vec<Vec<char>>, to_stack: u32, moving_crate: char){
    stacks[to_stack as usize].insert(0, moving_crate);
} 

fn move_crates_9000(stacks: &mut Vec<Vec<char>>, moves: &Vec<Vec<u32>>){
    for current_move in moves {
        for _ in 0..current_move[0]{
            let moving_crate = pop_stack(stacks, current_move[1] - 1);
            push_stack(stacks, current_move[2] - 1, moving_crate);
        }
    }
}

fn move_crates_9001(stacks: &mut Vec<Vec<char>>, moves: &Vec<Vec<u32>>){
    for current_move in moves {
        let mut moving_crates = vec![];
        for _ in 0..current_move[0]{
            moving_crates.push(pop_stack(stacks, current_move[1] - 1));
        }
        for i in 0..current_move[0]{
            push_stack(stacks, current_move[2] - 1, moving_crates[(current_move[0] - 1 - i) as usize]);
        }
    }
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> String {
    let mut top_crates = "".to_string();
    for i in 0..stacks.len(){
        top_crates.push(stacks[i][0]);
    }
    return top_crates;
}

pub fn run() {
    let lines = utils::lines_from_file("./src/day_05/input.txt").expect("Failed to read line from file");
    let mut stacks_9000 = parse_stacks(&lines, 9);
    let mut stacks_9001 = stacks_9000.clone();
    let moves = parse_moves(&lines);
    move_crates_9000(&mut stacks_9000, &moves);
    move_crates_9001(&mut stacks_9001, &moves);
    let top_crates_9000 = get_top_crates(&stacks_9000);
    let top_crates_9001 = get_top_crates(&stacks_9001);
    assert_eq!(top_crates_9000, "TPGVQPFDH");
    assert_eq!(top_crates_9001, "DMRDFRHHH");
}