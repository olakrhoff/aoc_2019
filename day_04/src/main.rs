use std::fs;
use std::io::BufRead;
use std::process;
use std::time::Instant;

const DATA_FILE_PATH: &str = "data.txt";
const TEST_DATA_FILE_PATH: &str = "test_data.txt";

const DEBUG: bool = false;
const PART_TWO: bool = false;

fn check_criteria(number: u32) -> bool
{
    let mut previous_digit: i16 = -1;
    let mut adjacent_equal = false;
    for digit_char in number.to_string().chars()
    {
        let digit = u32::from(digit_char) as i16;
        if digit < previous_digit
        {
            println!("Number: {}", number);
            return false;
        }
        if previous_digit == digit
        {
            adjacent_equal = true;
        }
        previous_digit = digit;
    }
    
    adjacent_equal
}

pub fn solve_1(start: u32, stop: u32) -> u32
{
    let mut counter = 0;

    for num in start..=stop
    {
        if check_criteria(num)
        {
            counter += 1;
        }
    }

    counter
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

    let mut start_number: u32 = 0;
    let mut stop_number: u32 = 0;

    for result in reader.lines()
    {
        match result
        {
            Ok(line) =>
            {
                // PROCESS THE LINE INPUT HERE
                let start_stop: Vec<_> = line.split('-').collect();
                
                for num in start_stop
                {
                    let num: u32 = match num.parse()
                    {
                        Ok(num) => num,
                        Err(_) =>
                        {
                            eprintln!("Error: Could not convert string to number in parsing");
                            process::exit(1);
                        }

                    };

                    if start_number == 0
                    {
                        start_number = num;
                    }
                    else if stop_number == 0
                    {
                        stop_number = num;
                    }
                    else
                    {
                        eprintln!("Error: More than two elements return from split in parsing.");
                        process::exit(1);
                    }
                }
            },
            Err(e) => 
            {
                eprintln!("Error: could not read line from file, {}", e);
                process::exit(1);
            }
        }
    }
    if start_number == 0 || stop_number == 0
    {
        eprintln!("Error: we expect both start and stop to differ from 0 (zero), please check your file");
        process::exit(1);
    }

    // DO THE PROCESSING OF THE DATA HERE

    let number_of_passwords = solve_1(start_number, stop_number);
    println!("Number of possible passwords: {}", number_of_passwords);

    let duration = start_time.elapsed();

    println!("Finished running in: {:.3?}", duration);
}
