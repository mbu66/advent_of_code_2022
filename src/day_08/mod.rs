#[path = "../utils/mod.rs"] mod utils;

fn reverse_forrest(forrest: &Vec<String>) -> Vec<String> {
    let mut reversed_forrest = vec![];
    for row in forrest {
        reversed_forrest.insert(0, row.chars().rev().collect::<String>());
    }
    return reversed_forrest;
}

fn parse_forrest(forrest: &Vec<String>, size: usize) -> Vec<Vec<bool>> {
    let mut visible_trees = vec![vec![false; size]; size];
    let mut max_height_col: Vec<i32> = vec![-1; size];
    for i in 0..size {
        let mut max_height_row = -1;
        for j in 0..size {
            let mut tree_is_visible = false;
            let tree_height = forrest[i].chars().nth(j).map(|c| c.to_digit(10).unwrap()).unwrap() as i32;
            if tree_height > max_height_row {
                max_height_row = tree_height;
                tree_is_visible = true;
            }
            if tree_height > max_height_col[j] {
                max_height_col[j]  = tree_height;
                tree_is_visible = true;
            }
            if tree_is_visible == true{
                visible_trees[i][j] = true ;
            }
        }
    }
    return visible_trees;
}

fn parse_lines_visible(forrest: &Vec<String>, size: usize) -> i32 {
    let reversed_forrest = reverse_forrest(forrest);
    let visible_trees_tl = parse_forrest(forrest, size);
    let visible_trees_br = parse_forrest(&reversed_forrest, size);
    
    let mut num_visible_trees = 0;
    for i in 0..size {
        for j in 0..size {
            if visible_trees_tl[i][j] || visible_trees_br[size - i - 1][size - j - 1]{
                num_visible_trees += 1;
            }
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
    let mut score = 0;
    for tree in trees {
        if tree < height{
            score +=1;
        }
        else {
            score +=1;
            break;
        }
    }
    return score;
}

fn parse_lines_scenic(forrest: &Vec<String>, size: usize) -> u32 {
    let mut max_scenic_score = 0;
    let transposed_forrest = transpose_forrest(forrest, size);
    for i in 0..size {
        for j in 0..size {
            let tree_height = forrest[i].chars().nth(j).map(|c| c.to_digit(10).unwrap()).unwrap() as u32;
            let mut trees_l = vec![];
            let mut trees_r = vec![];
            let mut trees_t = vec![];
            let mut trees_b = vec![];
            if i != 0 {
                trees_t = transposed_forrest[j][..i].chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
                trees_t.reverse();
            }
            if j != 0 {
                trees_l = forrest[i][..j].chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
                trees_l.reverse();
            }
            if i != size - 1{
                trees_b = transposed_forrest[j][(i+1)..].chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            }
            if j != size - 1{
                trees_r = forrest[i][(j+1)..].chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            }
            let score_l = scenic_score(&trees_l, &tree_height); 
            let score_r = scenic_score(&trees_r, &tree_height); 
            let score_t = scenic_score(&trees_t, &tree_height);
            let score_b = scenic_score(&trees_b, &tree_height); 

            let scenic_score = score_l * score_r * score_t * score_b;
            if max_scenic_score < scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }
    return max_scenic_score;
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_08/input.txt").expect("Failed to read line from file");
    let num_visible_trees = parse_lines_visible(&lines, 99);
    let max_scenic_score = parse_lines_scenic(&lines, 99);
    assert_eq!(num_visible_trees, 1851);
    assert_eq!(max_scenic_score, 574080);
}