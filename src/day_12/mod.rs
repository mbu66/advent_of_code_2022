#[path = "../utils/mod.rs"] mod utils;

fn propagate<const UP: bool >(topology: &Vec<Vec<i32>>, coord_a: (usize, usize),  coord_b: (usize, usize), active_coords: &mut Vec<(usize, usize)>, distances: &mut Vec<Vec<i32>>) {
    if distances[coord_b.0][coord_b.1] == 0{
        if (UP == true && topology[coord_b.0][coord_b.1] <= topology[coord_a.0][coord_a.1] + 1) ||
           (UP == false && topology[coord_b.0][coord_b.1] >= topology[coord_a.0][coord_a.1] - 1) {
            active_coords.push(coord_b.clone());
            distances[coord_b.0][coord_b.1] = distances[coord_a.0][coord_a.1] + 1;
        }
    }
}

fn parse_topology_down(topology: &Vec<Vec<i32>>, start: (usize, usize),) -> i32 {
    let mut distances = vec![vec![0; topology[0].len()]; topology.len()];
    let mut active_coords = vec![start];
    let mut end_reached = false;
    let mut end = (0, 0);
    while end_reached == false {
        let local_coords = active_coords.clone();
        active_coords.clear();
        for coord in local_coords {
            if coord.0 != distances.len() - 1 { propagate::<false>(&topology, coord, (coord.0 + 1, coord.1), &mut active_coords, &mut distances); }
            if coord.1 != distances[0].len() - 1 { propagate::<false>(&topology, coord, (coord.0, coord.1 + 1), &mut active_coords, &mut distances); }
            if coord.0 != 0 { propagate::<false>(&topology, coord, (coord.0 - 1, coord.1), &mut active_coords, &mut distances); }
            if coord.1 != 0 { propagate::<false>(&topology, coord, (coord.0, coord.1 - 1), &mut active_coords, &mut distances); }
            if coord == start { distances[start.0][start.1] = 1; }
            if topology[coord.0][coord.1] == 0 { end_reached = true; end = coord.clone(); }
        }
    }
    return distances[end.0][end.1];
}

fn parse_topology_up(topology: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut distances = vec![vec![0; topology[0].len()]; topology.len()];
    let mut active_coords = vec![start];
    let mut end_reached = false;
    while end_reached == false {
        let local_coords = active_coords.clone();
        active_coords.clear();
        for coord in local_coords {
            if coord.0 != distances.len() - 1 { propagate::<true>(&topology, coord, (coord.0 + 1, coord.1), &mut active_coords, &mut distances); }
            if coord.1 != distances[0].len() - 1 { propagate::<true>(&topology, coord, (coord.0, coord.1 + 1), &mut active_coords, &mut distances); }
            if coord.0 != 0 { propagate::<true>(&topology, coord, (coord.0 - 1, coord.1), &mut active_coords, &mut distances); }
            if coord.1 != 0 { propagate::<true>(&topology, coord, (coord.0, coord.1 - 1), &mut active_coords, &mut distances); }
            if coord == start { distances[start.0][start.1] = 1; }
            if coord == end { end_reached = true; }
        }
    }
    return distances[end.0][end.1];
}

fn parse_lines(lines: Vec<String>) -> ((usize, usize), (usize, usize), Vec<Vec<i32>>) {
    let (mut start, mut end, mut topology) = ((0, 0), (0, 0), vec![]);
    for (i, line) in lines.iter().enumerate() {
        topology.push(line.chars().map(|x| x as i32 - 97).collect::<Vec<i32>>());
        for (j, height) in topology.last().unwrap().iter().enumerate() {
            if *height == -14 { start = (i, j); }
            if *height == -28 { end = (i, j); }
        }
    }
    topology[start.0][start.1] = 0;
    topology[end.0][end.1] = 26;
    return (start, end, topology);
} 

pub fn run() {
    let lines = utils::lines_from_file("./src/day_12/input.txt").expect("Failed to read line from file");
    let (start, end, topology) = parse_lines(lines);
    let distance_up = parse_topology_up(&topology, start, end);
    let distance_down = parse_topology_down(&topology, end);
    assert_eq!(distance_up, 484);
    assert_eq!(distance_down, 478);
}