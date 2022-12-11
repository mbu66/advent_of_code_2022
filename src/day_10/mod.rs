#[path = "../utils/mod.rs"] mod utils;

fn _render_image(image: &Vec<bool>) {
    for (i, pixel) in image.iter().enumerate() {
        match pixel {
            true => print!("# "),
            false => print!(". "),
        }
        if (i + 1) % 40 == 0 { 
            print!("\n"); 
        }
    }
}

fn get_strength(signal: &Vec<i32>) -> i32 {
    let mut signal_strength = 0;
    for i in 0..6 { 
        signal_strength += (20 + (40 * i)) as i32 * signal[19 + (40 * i)];
    }
    return signal_strength;
}

fn parse_sprite(signal: &Vec<i32>) -> Vec<bool> {
    let mut image = vec![];
    for (i, position) in signal.iter().enumerate() {
        image.push(position - 1 <= i as i32 % 40  && i as i32 % 40 <= position + 1);
    }
    return image;
}

fn parse_lines(lines: &Vec<String>) -> Vec<i32> {
    let mut signal = vec![1];
    for line in lines {
        signal.push(signal.last().copied().unwrap());
        if line != "noop" {
            signal.push(signal.last().copied().unwrap() + line.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap());
        }
    }
    return signal;
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_10/input.txt").expect("Failed to read line from file");
    let signal = parse_lines(&lines);
    let signal_strength = get_strength(&signal);
    let _image = parse_sprite(&signal);
    assert_eq!(signal_strength, 13220);
    //Rendering image should print the letters RUAKHBEK
    // _render_image(&_image);
}