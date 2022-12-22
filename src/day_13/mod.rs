#[path = "../utils/mod.rs"] mod utils;


fn parse_pair(signal_1: &mut String, signal_2: &mut String) -> bool {
    let mut correct_order = true;
    while correct_order && signal_1.len() != 0 {
        if signal_2.len() == 0 {correct_order = false; break;}

        if signal_1.chars().nth(0).unwrap() == signal_2.chars().nth(0).unwrap() { signal_1.remove(0); signal_2.remove(0); continue; }

        if signal_1.chars().nth(0).unwrap() == ',' { correct_order = false; break; }
        if signal_2.chars().nth(0).unwrap() == ',' { break; }

        if signal_1.chars().nth(0).unwrap() == ']' { signal_2.remove(0); continue;  }
        if signal_2.chars().nth(0).unwrap() == ']' { correct_order = false; break;}

        if signal_1.chars().nth(0).unwrap() == '[' { signal_1.remove(0); continue;  }
        if signal_2.chars().nth(0).unwrap() == '[' { signal_2.remove(0); continue;  }


        if signal_1.chars().nth(0).unwrap().to_digit(10).unwrap() < signal_2.chars().nth(0).unwrap().to_digit(10).unwrap() { signal_1.remove(0); signal_2.remove(0); break; }
        correct_order = false;
        break;
    }
    return correct_order;
}

fn parse_pairs(lines: Vec<String>) -> usize {
    let mut correct_indices = vec![];
    for i in 0..((lines.len() + 1) / 3){
        let mut signal_1 = lines[3 * i].clone();
        let mut signal_2 = lines[(3 * i) + 1].clone();
        if parse_pair(&mut signal_1, &mut signal_2) { correct_indices.push(i + 1); }
    }
    return correct_indices.iter().sum();
}


pub fn run() {
    let lines = utils::lines_from_file("./src/day_13/input.txt").expect("Failed to read line from file");
    let num_correct_pairs = parse_pairs(lines);
    println!("{num_correct_pairs}");
    assert_eq!(parse_pairs(vec![String::from("[1,1,3,1,1]"), String::from("[1,1,5,1,1]") ]), 1);
    assert_eq!(parse_pairs(vec![String::from("[[1],[2,3,4]]"), String::from("[[1],4]") ]), 1);
    assert_eq!(parse_pairs(vec![String::from("[9]"), String::from("[[8,7,6]]") ]), 0);
    assert_eq!(parse_pairs(vec![String::from("[[4,4],4,4]"), String::from("[[4,4],4,4,4]") ]), 1);
    assert_eq!(parse_pairs(vec![String::from("[7,7,7,7]"), String::from("[7,7,7]") ]), 0);
    assert_eq!(parse_pairs(vec![String::from("[]"), String::from("[3]") ]), 1);
    assert_eq!(parse_pairs(vec![String::from("[[[]]]"), String::from("[[]]") ]), 0);
    assert_eq!(parse_pairs(vec![String::from("[1,[2,[3,[4,[5,6,7]]]],8,9]"), String::from("[1,[2,[3,[4,[5,6,0]]]],8,9]") ]), 0);
    assert_eq!(parse_pairs(vec![String::from("[1,2,[3,4]]"), String::from("[1,2,[3,4,5]]") ]), 1);
    assert_eq!(parse_pairs(vec![String::from("[1,2,[3,4,5]]"), String::from("[1,2,[3,4]]") ]), 0);


    // assert_eq!(, );
}