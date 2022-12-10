#[path = "../utils/mod.rs"] mod utils;

fn render_image(image: &Vec<bool>) {
    for (i, pixel) in image.iter().enumerate() {
        match pixel {
            true => print!("# "),
            false => print!(". "),
        }
        if (i + 1) % 40 == 0{
            print!("\n");
        }
    }
}

fn get_strength(signal: &Vec<i32>) -> i32 {
    let mut signal_strength = 0;
    signal_strength += 20 * signal[19];
    signal_strength += 60 * signal[59];
    signal_strength += 100 * signal[99];
    signal_strength += 140 * signal[139];
    signal_strength += 180 * signal[179];
    signal_strength += 220 * signal[219];
    return signal_strength;
}

fn parse_sprite(signal: &Vec<i32>) -> Vec<bool> {
    let mut image = vec![];
    for (i, position) in signal.iter().enumerate() {
        if position - 1 <= i as i32 % 40  && i as i32 % 40 <= position + 1 {
            image.push(true);
        } else {
            image.push(false);
        }
    }
    return image;
}

fn parse_lines(lines: &Vec<String>) -> Vec<i32> {
    let mut signal = vec![1];
    for line in lines {
        if line == "noop" {
            signal.push(signal.last().copied().unwrap());
        } else {
            signal.push(signal.last().copied().unwrap());
            signal.push(signal.last().copied().unwrap() + line.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap());
        }
    }

    return signal;
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_10/input.txt").expect("Failed to read line from file");
    let signal = parse_lines(&lines);
    let signal_strength = get_strength(&signal);
    let image = parse_sprite(&signal);
    assert_eq!(signal_strength, 13220);
    render_image(&image);
    //Rendering image should print the letters RUAKHBEK
}