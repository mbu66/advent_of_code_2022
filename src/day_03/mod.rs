#[path = "../utils/mod.rs"] mod utils;

fn parse_lines(lines: &Vec<String>) -> Vec<u32> {
   let mut priority_scores = vec![];
    for line in lines {
        let size = line.len() / 2;
        let mut bag1 = vec![];
        let mut shared_item = 0;

        for (i, c) in line.chars().enumerate() {
            if i < size {
                bag1.push(c);
            }
            else {
                if bag1.contains(&c)
                {
                    shared_item = c as u32;
                    break;
                }
            }
        }
        if shared_item < 91 {
            priority_scores.push(shared_item - 38);
        }
        else {

            priority_scores.push(shared_item - 96);
        }
    }
    return priority_scores;
} 

fn parse_lines2(lines: &Vec<String>) -> Vec<u32> {
    let mut priority_scores = vec![];
    let mut letters = vec![];
    let mut letters2 = vec![];
    let mut letters3 = vec![];

    let mut shared_item = 0;
    for (i, line) in lines.iter().enumerate(){
        if (i + 3) % 3 == 0{
            for c in line.chars() {
                letters.push(c);
            }
        }
        if (i + 2) % 3 == 0{
            for c in line.chars() {
                if letters.contains(&c){
                    letters2.push(c);
                }
            }
        }
        if (i + 1) % 3 == 0{
            for c in line.chars() {
                if letters2.contains(&c){
                    letters3.push(c);
                    shared_item = c as u32;
                    break;
                }
            }
            if shared_item < 91{
                priority_scores.push(shared_item - 38);
            }
            else{
                priority_scores.push(shared_item - 96);
            }
            letters.clear();
            letters2.clear();
            letters3.clear();

        }
    }
    return priority_scores;
}

pub fn run() {
    let lines = utils::lines_from_file("./src/day_03/input.txt").expect("Failed to read line from file");
    let priority_scores1 = parse_lines(&lines);
    let priority_scores2 = parse_lines2(&lines);
    assert_eq!(priority_scores1.iter().sum::<u32>(), 8493);
    assert_eq!(priority_scores2.iter().sum::<u32>(), 2552);
}