#[path = "../utils/mod.rs"] mod utils;

fn parse_forrest(forrest: &Vec<String>, size: usize) -> Vec<Vec<bool>> {
    let mut visible_trees = vec![vec![false; size]; size];
    let mut max_height_col = vec![-1; size];
    for i in 0..size {
        let mut max_height_row = -1;
        for j in 0..size {
            let tree_height = forrest[i].chars().nth(j).map(|c| c.to_digit(10).unwrap()).unwrap() as i32;
            if tree_height > max_height_row {
                max_height_row = tree_height;
                visible_trees[i][j] = true ;
            }
            if tree_height > max_height_col[j] {
                max_height_col[j]  = tree_height;
                visible_trees[i][j] = true ;
            }
        }
    }
    return visible_trees;
}

fn parse_lines_for_visible_trees(forrest: &Vec<String>, size: usize) -> i32 {
    let mut reversed_forrest = vec![];
    for row in forrest {
        reversed_forrest.insert(0, row.chars().rev().collect::<String>());
    }
    let visible_trees_tl = parse_forrest(forrest, size);
    let visible_trees_br = parse_forrest(&reversed_forrest, size);
    
    let mut num_visible_trees = 0;
    for i in 0..size {
        for j in 0..size {
            if visible_trees_tl[i][j] || visible_trees_br[size - i - 1][size - j - 1]{ num_visible_trees += 1;}
        }
    }
    return num_visible_trees;
} 

fn transpose_forrest(forrest: &Vec<String>, size: usize) -> Vec<String> {
    let mut transposed_forrest = vec![String::from(""); size];
    for i in 0..size {
        let trees = forrest[i].chars().collect::<Vec<char>>();
        for (j, tree) in trees.iter().enumerate() {
            transposed_forrest[j].push(tree.clone());
        }
    }
    return transposed_forrest;
}

fn scenic_score(trees: &Vec<u32>, height: &u32) -> u32{
    if trees.len() == 0 {return 0;}
    return 1 + trees.iter().position(|&x| x >= *height).unwrap_or_else(||trees.len() - 1) as u32;
}

fn parse_lines_for_scenic_score(forrest: &Vec<String>, size: usize) -> u32 {
    let mut max_scenic_score = 0;
    let transposed_forrest = transpose_forrest(forrest, size);
    for i in 0..size {
        for j in 0..size {
            let trees_l = forrest           [i][..j]    .chars().map(|c| c.to_digit(10).unwrap()).rev().collect::<Vec<u32>>();
            let trees_t = transposed_forrest[j][..i]    .chars().map(|c| c.to_digit(10).unwrap()).rev().collect::<Vec<u32>>();
            let trees_r = forrest           [i][(j+1)..].chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            let trees_b = transposed_forrest[j][(i+1)..].chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            let tree_height = forrest[i].chars().nth(j).map(|c| c.to_digit(10).unwrap()).unwrap() as u32;
            let scenic_score = scenic_score(&trees_l, &tree_height) * scenic_score(&trees_t, &tree_height) * 
                               scenic_score(&trees_r, &tree_height) * scenic_score(&trees_b, &tree_height);
            if max_scenic_score < scenic_score { max_scenic_score = scenic_score}
        }
    }
    return max_scenic_score;
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_08/input.txt").expect("Failed to read line from file");
    let num_visible_trees = parse_lines_for_visible_trees(&lines, 99);
    let max_scenic_score = parse_lines_for_scenic_score(&lines, 99);
    assert_eq!(num_visible_trees, 1851);
    assert_eq!(max_scenic_score, 574080);
}