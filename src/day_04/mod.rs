#[path = "../utils/mod.rs"] mod utils;

fn parse_lines(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut section_ids = vec![];
    for line in lines {
        let split = line.split(",").collect::<Vec<&str>>();
        let mut first_ids = split[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut second_ids = split[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        first_ids.append(&mut second_ids);
        section_ids.push(first_ids);
    }
    return section_ids;
} 

fn count_nested_id_pairs(id_pairs: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for id_pair in id_pairs {
        if (id_pair[0] <= id_pair[2] && id_pair[1] >= id_pair[3]) ||
            (id_pair[2] <= id_pair[0] && id_pair[3] >= id_pair[1]) { 
                count += 1;
        }
    }
    return count;
} 

fn count_overlapping_id_pairs(id_pairs: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for id_pair in id_pairs {
        if (id_pair[0] <= id_pair[2] && id_pair[2] <= id_pair[1]) ||
           (id_pair[0] <= id_pair[3] && id_pair[3] <= id_pair[1]) ||
           (id_pair[2] <= id_pair[0] && id_pair[0] <= id_pair[3]) ||
           (id_pair[2] <= id_pair[1] && id_pair[1] <= id_pair[3]) { 
                count += 1;
        }
    }
    return count;
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_04/input.txt").expect("Failed to read line from file");
    let section_ids = parse_lines(lines);
    let nested_count = count_nested_id_pairs(&section_ids);
    let overlapping_count = count_overlapping_id_pairs(&section_ids);

    assert_eq!(nested_count, 471);
    assert_eq!(overlapping_count, 888);
}