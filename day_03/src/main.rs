use std::fs;
use std::io::BufRead;
use std::process;
use std::time::Instant;
use regex::Regex;
use std::collections::BTreeSet;

const DATA_FILE_PATH: &str = "data.txt";
const TEST_DATA_FILE_PATH: &str = "test_data.txt";

const DEBUG: bool = false;
const PART_TWO: bool = false;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
struct Coord
{
    x: i32,
    y: i32
}

impl Coord
{
    pub fn make_move(&mut self, dir: char, length: u32)
    {
        match dir
        {
            'R' => self.x += length as i32, 
            'U' => self.y += length as i32,
            'L' => self.x -= length as i32,
            'D' => self.y -= length as i32,
            _ => 
            {
                eprintln!("Error: Direction not recognised");
                process::exit(1);
            }
        }
    }
}

#[derive(Eq, PartialEq)]
struct CoordByX
{
    coord: Coord,
    index: usize,
}

impl Ord for CoordByX
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.coord.x
            .cmp(&other.coord.x)
            .then(self.coord.y.cmp(&other.coord.y))
    }
}

impl PartialOrd for CoordByX
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
    {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct CoordByY
{
    coord: Coord,
    index: usize,
}

impl Ord for CoordByY
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.coord.y
            .cmp(&other.coord.y)
            .then(self.coord.x.cmp(&other.coord.x))
    }
}

impl PartialOrd for CoordByY
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
    {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone)]
struct Segment
{
    start: Coord,
    stop: Coord,
}

impl Segment
{
    fn is_vertical(&self) -> bool
    {
        self.start.x == self.stop.x
    }

    fn is_horisontal(&self) -> bool
    {
        !self.is_vertical()
    }

    fn x_range(&self) -> (i32, i32)
    {
        (self.start.x.min(self.stop.x), self.start.x.max(self.stop.x))
    }
    
    fn y_range(&self) -> (i32, i32)
    {
        (self.start.y.min(self.stop.y), self.start.y.max(self.stop.y))
    }
}

fn convert_move_to_coord(last_coord: &Coord, move_str: &String) -> Coord
{
    let re = Regex::new(r"^([RLUD])(\d+)$").unwrap();

    let caps = re
        .captures(&move_str)
        .unwrap_or_else(||
        {
            eprintln!("Invalid move format: {}", move_str);
            process::exit(1);
        });

    let dir: char = caps[1].chars().next().unwrap();
    let length: u32 = caps[2].parse().unwrap();


    let mut coord = *last_coord;
    coord.make_move(dir, length);

    return coord;
}

fn parse_line_to_coords(line: String) -> Vec<Coord>
{
    let mut coords: Vec<Coord> = vec!();
    coords.push(Coord { x: 0, y: 0 });
    for move_str in line.split(',')
    {
        coords.push(convert_move_to_coord(coords.last().unwrap(), &move_str.to_string()));
    }

    return coords; 
}

fn manhatten_distance(coord: Coord) -> u32
{
    (coord.x.abs() + coord.y.abs()) as u32
}

fn check_overlap(a: Segment, b: Segment) -> Vec<Coord>
{
    let mut result = vec!();

    match (a.is_vertical(), b.is_vertical())
    {
        (true, false) => 
        {
            let x = a.start.x;
            let y = b.start.y;

            let (ay1, ay2) = a.y_range();
            let (bx1, bx2) = b.x_range();

            if ay1 <= y && y <= ay2 && bx1 <= x && x <= bx2
            {
                result.push(Coord {x, y});
            }
        }
        (false, true) =>
        {
            return check_overlap(b, a);
        }
        (true, true) => 
        {
            if a.start.x != b.start.x
            {
                return result;
            }

            let (ay1, ay2) = a.y_range();
            let (by1, by2) = b.y_range();

            let low = ay1.max(by1);
            let high = ay2.min(by2);

            if low <= high
            {
                for y in low..=high
                {
                    result.push(Coord {x: a.start.x, y});
                }
            }
        }
        (false, false) => 
        {
            if a.start.y != b.start.y
            {
                return result;
            }

            let (ax1, ax2) = a.x_range();
            let (bx1, bx2) = b.x_range();

            let low = ax1.max(bx1);
            let high = ax2.min(bx2);

            if low <= high
            {
                for x in low..=high
                {
                    result.push(Coord {x, y: a.start.y});
                }
            }
        }
    }

    result
}

fn solve(wire_a: Vec<Coord>, wire_b: Vec<Coord>) -> u32
{
    let set_by_x: BTreeSet<CoordByX> = wire_a
		.iter()
        .enumerate()
		.map(|(index, &coord)| CoordByX {coord, index})
        .collect();
    let set_by_y: BTreeSet<CoordByY> = wire_a
		.iter()
		.enumerate()
		.map(|(index, &coord)| CoordByY {coord, index})
		.collect();

    // Step 1: Find all intersections
    let mut intersections: Vec<Coord> = vec!();

    for current_index in 0..wire_b.len() - 1
    {
        let start = wire_b[current_index];
        let stop = wire_b[current_index + 1];
        let b_segment = Segment {start, stop};
        
        if b_segment.is_horisontal()
        {
            let y = start.y;
            let x_min = start.x.min(stop.x);
            let x_max = start.x.max(stop.x);

            let candidates = set_by_x.range(
                CoordByX
                {
                    coord: Coord {x: x_min, y},
                    index: 0,
                }..=CoordByX
                {
                    coord: Coord {x: x_max, y},
                    index: 0,
                });
            
            for candidate in candidates
            {
                let index = candidate.index;
                if index > 0
                {
                    let a_segment = Segment 
                    {
                        start: wire_a[index - 1],
                        stop: wire_a[index]
                    };

                    intersections.extend(check_overlap(a_segment, b_segment));
                }

                if index < wire_a.len() - 1
                {
                    let a_segment = Segment 
                    {
                        start: wire_a[index],
                        stop: wire_a[index + 1]
                    };

                    intersections.extend(check_overlap(a_segment, b_segment));
                }
            }
        }
        else // is vertical
        {
            let x = start.x;
            let y_min = start.y.min(stop.y);
            let y_max = start.y.max(stop.y);

            let candidates = set_by_y.range(
                CoordByY
                {
                    coord: Coord {x, y: y_min},
                    index: 0
                }..=CoordByY
                {
                    coord: Coord {x, y: y_max},
                    index: 0
                });

            for candidate in candidates
            {
                let index = candidate.index;

                if index > 0 
                {
                    let a_segment = Segment
                    {
                        start: wire_a[index - 1],
                        stop: wire_a[index]
                    };

                    intersections.extend(check_overlap(a_segment, b_segment));
                }

                if index < wire_a.len() - 1
                {
                    let a_segment = Segment
                    {
                        start: wire_a[index],
                        stop: wire_a[index + 1]
                    };

                    intersections.extend(check_overlap(a_segment, b_segment));
                }
            }

        }
    }

    intersections.retain(|c| !(c.x == 0 && c.y == 0));

    // Step 2: Find the closest in Manhatten distance, and return it
    let mut shortest_distance: u32 = u32::MAX;

    for intersection in intersections
    {
        let distance: u32 = manhatten_distance(intersection);
        if distance < shortest_distance
        {
            shortest_distance = distance;
        }
    }

    shortest_distance
}

fn main()
{
    println!("Starting...");
    let filename = if DEBUG { TEST_DATA_FILE_PATH } else { DATA_FILE_PATH };
    let file = match fs::File::open(filename)
    {
        Ok(f) => f,
        Err(e) => 
        {
            eprintln!("Error: could not load file {}, {}", filename, e);
            process::exit(1);
        }
    };
    let reader = std::io::BufReader::new(file);

    let start_time = Instant::now();


    let mut wires: Vec<Vec<Coord>> = vec!();

    for result in reader.lines()
    {
        match result
        {
            Ok(line) =>
            {
                wires.push(parse_line_to_coords(line));
            },
            Err(e) => 
            {
                eprintln!("Error: could not read line from file, {}", e);
                process::exit(1);
            }
        }
    }

    if wires.len() != 2
    {
        eprintln!("Error: we expect exactly two wires, but got: {}", wires.len());
        process::exit(1);
    }

    let distance: u32 = solve(wires[0].to_vec(), wires[1].to_vec());

    let duration = start_time.elapsed();

    println!("Manhatten distance from start to closest intersection is: {}", distance);
    println!("Finished running in: {:.3?}", duration);
}
