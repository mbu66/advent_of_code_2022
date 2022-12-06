#[path = "../utils/mod.rs"] mod utils;

fn parse_lines_for_score(lines: &Vec<String>) -> Vec<i32> {
    let mut scores = vec![];
    for line in lines
    {
        match line.as_str(){
            "A X"=>scores.push(4), // 1 + 3
            "A Y"=>scores.push(8), // 2 + 6
            "A Z"=>scores.push(3), // 3 + 0
            "B X"=>scores.push(1), // 1 + 0
            "B Y"=>scores.push(5), // 2 + 3
            "B Z"=>scores.push(9), // 3 + 6
            "C X"=>scores.push(7), // 1 + 6
            "C Y"=>scores.push(2), // 2 + 0
            "C Z"=>scores.push(6), // 3 + 3
            _=>assert!(false),
        }
    }
    return scores;
} 

fn parse_lines_for_game(games_in: &Vec<String>) -> Vec<String> {
    let mut games_out = vec![];
    for game in games_in
    {
        match game.as_str(){
            "A X"=>games_out.push("A Z".to_string()), // loose + A
            "A Y"=>games_out.push("A X".to_string()), // draw + A
            "A Z"=>games_out.push("A Y".to_string()), // win + A
            "B X"=>games_out.push("B X".to_string()), // loose + B
            "B Y"=>games_out.push("B Y".to_string()), // draw + B
            "B Z"=>games_out.push("B Z".to_string()), // win + B
            "C X"=>games_out.push("C Y".to_string()), // loose + C
            "C Y"=>games_out.push("C Z".to_string()), // draw + C
            "C Z"=>games_out.push("C X".to_string()), // win + C
            _=>assert!(false),
        }
    }
    return games_out;
} 
pub fn run() {
    let games1 = utils::lines_from_file("./src/day_02/input.txt").expect("Failed to read line from file");
    let scores1 = parse_lines_for_score(&games1);
    let games2 = parse_lines_for_game(&games1);
    let scores2 = parse_lines_for_score(&games2);
    assert_eq!(scores1.iter().sum::<i32>(), 13221);
    assert_eq!(scores2.iter().sum::<i32>(), 13131);
}