#[path = "../utils/mod.rs"] mod utils;

fn parse_forrest(forrest_in: &Vec<String>) -> (u32, u32) {
    let mut visible_trees = 0;
    let mut max_scenic_score = 0;
    let mut forrest = vec![vec![0; forrest_in.len()]; forrest_in.len()];
    let mut forrest_t = vec![vec![0; forrest_in.len()]; forrest_in.len()];
    for i in 0..forrest_in.len() {
        for (j, tree) in forrest_in[i].chars().enumerate() {
            forrest[i][j] = tree.to_digit(10).unwrap();
            forrest_t[j][i] = tree.to_digit(10).unwrap();
        }
    }
    for i in 0..forrest.len() {
        for j in 0..forrest.len() {
            let tree_height = forrest[i][j];
            let mut trees_l = forrest[i][..j].to_vec();
            let mut trees_t = forrest_t[j][..i].to_vec();
            let trees_r = forrest[i][(j+1)..].to_vec();
            let trees_b = forrest_t[j][(i+1)..].to_vec();
            trees_l.reverse();
            trees_t.reverse();
            let scenic_score = scenic_score(&trees_l, &tree_height) * scenic_score(&trees_t, &tree_height) * 
                               scenic_score(&trees_r, &tree_height) * scenic_score(&trees_b, &tree_height);
            let is_visible = is_visible(&trees_l, &tree_height) || is_visible(&trees_t, &tree_height) || 
                             is_visible(&trees_r, &tree_height) || is_visible(&trees_b, &tree_height);
            if max_scenic_score < scenic_score { max_scenic_score = scenic_score}
            if is_visible { visible_trees += 1; }
        }
    }
    return (visible_trees, max_scenic_score);
}

fn is_visible(trees: &Vec<u32>, height: &u32) -> bool{
    if trees.len() == 0 {return true;}
    return trees.iter().any(|&x| x >= *height) == false;
}

fn scenic_score(trees: &Vec<u32>, height: &u32) -> u32{
    if trees.len() == 0 {return 0;}
    return 1 + trees.iter().position(|&x| x >= *height).unwrap_or_else(||trees.len() - 1) as u32;
}

pub fn run() {
    let lines = utils::lines_from_file("./src/day_08/input.txt").expect("Failed to read line from file");
    let (num_visible_trees, max_scenic_score) = parse_forrest(&lines);
    assert_eq!(num_visible_trees, 1851);
    assert_eq!(max_scenic_score, 574080);
}