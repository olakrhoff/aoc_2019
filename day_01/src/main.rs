use std::fs;
use std::io::BufRead;
use std::process;
use std::time::Instant;

const DATA_FILE_PATH: &str = "data.txt";
const TEST_DATA_FILE_PATH: &str = "test_data.txt";

const DEBUG: bool = false;
const PART_TWO: bool = false;

fn parse_line_to_number(line: &str) -> i32
{
    match line.trim().parse()
    {
        Ok(n) => n,
        Err(e) => 
        {
            eprintln!("Error: Failed to parse line '{}' to number, error: {}", line, e);
            process::exit(1);
        }
    }
}

fn process_number(num: i32) -> i32
{
    let decimal: f32 = num as f32;

    let decimal = decimal / 3.0;

    let rounded: i32 = decimal as i32;

    rounded - 2
}

fn process_number_recursive(num: i32) -> i32
{
    let fuel: i32 = process_number(num);

    if fuel > 0
    {
        let temp = fuel + process_number_recursive(fuel);
        //println!("Temp: {}", temp);
        return temp;
    }

    if fuel > 0 { fuel } else { 0 }
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

    let mut numbers: Vec<i32> = Vec::new();

    for result in reader.lines()
    {
        match result
        {
            Ok(line) =>
            {
                let number: i32 = parse_line_to_number(&line); 
                numbers.push(number);
            },
            Err(e) => 
            {
                eprintln!("Error: could not read line from file, {}", e);
                process::exit(1);
            }
        }
    }


    let mut sum: i32 = 0;
    if !PART_TWO
    {
        for num in numbers
        {
            sum += process_number(num);
        }
    }
    else // PART_TWO
    {
        for num in numbers
        {
            sum += process_number_recursive(num);
        }
    }

    let duration = start_time.elapsed();


    println!("The sum of the processed numbers are: {}", sum);


    println!("Finished running in: {:.3?}", duration);
}
