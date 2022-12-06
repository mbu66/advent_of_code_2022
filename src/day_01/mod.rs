#[path = "../utils/mod.rs"] mod utils;

fn parse_lines(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut inventories = vec![];
    let mut temp_inventory = vec![];
    for line in lines {
        
        if line.is_empty() {
            inventories.push(temp_inventory.clone());
            temp_inventory.clear();
        }
        else {
            temp_inventory.push(line.parse::<i32>().unwrap());
        }
    }
    return inventories;

} 

fn parse_inventories(inventories: Vec<Vec<i32>>) -> (i32, i32, i32) {
    let mut max_calories_1 = 0;
    let mut max_calories_2 = 0;
    let mut max_calories_3 = 0;
    for inventory in inventories {
        let num_calories = inventory.iter().sum();
        if num_calories > max_calories_1{
            max_calories_3 = max_calories_2;
            max_calories_2 = max_calories_1;
            max_calories_1 = num_calories;
        }
        else if num_calories > max_calories_2{
            max_calories_3 = max_calories_2;
            max_calories_2 = num_calories;
        }
        else if num_calories > max_calories_3{
            max_calories_3 = num_calories;
        }
    }
    return (max_calories_1, max_calories_2, max_calories_3);
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_01/input.txt").expect("Failed to read line from file");
    let inventories = parse_lines(lines);
    let calories = parse_inventories(inventories);

    assert_eq!(calories.0, 72240);
    assert_eq!(calories.1, 69625);
    assert_eq!(calories.2, 69092);
    assert_eq!(calories.0 + calories.1 + calories.2, 210957);
}