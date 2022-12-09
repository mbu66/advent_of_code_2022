#[path = "../utils/mod.rs"] mod utils;

fn get_move(head_position: &(i32, i32), tail_position: &(i32, i32)) -> (i32, i32) {
    let diff = (head_position.0 - tail_position.0, head_position.1 - tail_position.1);
    let mut tail_move = (0, 0);
    if i32::abs(diff.0) == 2 {
        if i32::abs(diff.1) == 0 {
            tail_move = (1, 0);
        } else {
            tail_move = (1, 1);
        }
    }
    if i32::abs(diff.1) == 2 {
        if i32::abs(diff.0) == 0 {
            tail_move = (0, 1);
        } else {
            tail_move = (1, 1);
        }
    }
    tail_move.0 = tail_move.0 * i32::signum(diff.0);
    tail_move.1 = tail_move.1 * i32::signum(diff.1);
    return tail_move;
}


fn parse_lines(lines: &Vec<String>) -> (i32, i32) {
    let mut rope_position = vec![(0, 0); 10];
    
    let mut visitied_positions = vec![std::collections::HashSet::new(); 2];
    visitied_positions[0].insert((0, 0));
    visitied_positions[1].insert((0, 0));

    let mut num_positions_visited = (1, 1);

    for line in lines {
        let split = line.split(' ').collect::<Vec<&str>>();
        for _ in 0..split[1].parse::<i32>().unwrap() {
            match split[0] {
                x if x == "L" => rope_position[0].0 -= 1,
                x if x == "R" => rope_position[0].0 += 1,
                x if x == "U" => rope_position[0].1 += 1,
                x if x == "D" => rope_position[0].1 -= 1,
                _ => assert!(false)
            }
            let mut tail_move;
            for i in 0..9 {
                tail_move = get_move(&rope_position[i], &rope_position[i+1]);
                if tail_move == (0, 0) {break;}
                rope_position[i+1].0 += tail_move.0;
                rope_position[i+1].1 += tail_move.1;
            }

            if visitied_positions[0].insert(rope_position[1]) {
                num_positions_visited.0 += 1;
            }
            if visitied_positions[1].insert(rope_position[9]) {
                num_positions_visited.1 += 1;
            }
        }
    }
    return num_positions_visited;
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_09/input.txt").expect("Failed to read line from file");
    let num_positions_visited = parse_lines(&lines);
    assert_eq!(num_positions_visited.0, 6209);
    assert_eq!(num_positions_visited.1, 2460);
}