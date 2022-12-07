#[path = "../utils/mod.rs"] mod utils;

fn get_marker_index(line: &String, packet_size: usize) -> i32 {
    let mut packet = vec![];
    let mut index = -1;
    for (i, c) in line.chars().enumerate()  {
        packet.push(c);
        if packet.len() < packet_size {
            continue;
        }
        if packet.len() == (packet_size + 1){
            packet.remove(0);
        }
        let mut uniq = std::collections::HashSet::new();
        if packet.clone().into_iter().all(move |x| uniq.insert(x)){
           index = (i as i32) + 1;
           break; 
        }
    }
    return index;
} 


pub fn run() {
    let line = utils::lines_from_file("./src/day_06/input.txt").expect("Failed to read line from file");
    let packet_index = get_marker_index(&line[0], 4);
    let message_index = get_marker_index(&line[0], 14);
    assert_eq!(packet_index, 1598);
    assert_eq!(message_index, 2414);
}