use std::fs;
use std::io::BufRead;
use std::process;
use std::time::Instant;

const DATA_FILE_PATH: &str = "data.txt";
const TEST_DATA_FILE_PATH: &str = "test_data.txt";

const DEBUG: bool = false;
const PART_TWO: bool = false;

fn parse_line_to_numbers(line: &str) -> Vec<u32>
{
    let mut numbers: Vec<u32> = Vec::new();

    for n in line.split(',')
    {
        match n.parse()
        {
            Ok(valid) => 
            {
                numbers.push(valid);
            }
            Err(e) => 
            {
                eprintln!("Could not convert '{}' to a u32, error: {}", n, e);
                process::exit(1);
            }
        }
    }

    numbers
}

fn run_program(program: &mut Vec<u32>)
{
    let mut index = 0;
    
    loop
    {
        println!("INDEX: {}", index);
        match program[index]
        {
            1 => // Addition
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];
                let index3 = program[index + 3];

                let val1 = program[index1 as usize];
                let val2 = program[index2 as usize];

                let result = val1 + val2;
                program[index3 as usize] = result;

                println!("ADD {} {} => {}", val1, val2, result);
                index += 4;
            }
            2 => // Multiplication
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];
                let index3 = program[index + 3];

                let val1 = program[index1 as usize];
                let val2 = program[index2 as usize];

                let result = val1 * val2;
                program[index3 as usize] = result;

                println!("MUL {} {} => {}", val1, val2, result);
                index += 4;
            }
            99 => // Exit program
            {
                println!("EXIT");
                break;
            }
            other => // Catch all
            {
                eprintln!("Error: Opcode '{}' not recognised", other);
                process::exit(1);
            }
        }
    }
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

    let mut program: Vec<u32> = Vec::new(); 

    for result in reader.lines()
    {
        match result
        {
            Ok(line) =>
            {
               program = parse_line_to_numbers(&line); 
            },
            Err(e) => 
            {
                eprintln!("Error: could not read line from file, {}", e);
                process::exit(1);
            }
        }
    }

    // DO THE PROCESSING OF THE DATA HERE
    if !DEBUG
    {
        program[1] = 12;
        program[2] = 2;
    }
    run_program(&mut program);

    let duration = start_time.elapsed();

    for num in &program
    {
        print!("{},", num);
    }
       
    println!("");

    println!("The result in postion 0 is: {}", program[0]);

    println!("Finished running in: {:.3?}", duration);
}
