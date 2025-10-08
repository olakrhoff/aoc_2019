use std::fs;
use std::io::BufRead;
use std::process;
use std::time::Instant;

const DATA_FILE_PATH: &str = "data.txt";
const TEST_DATA_FILE_PATH: &str = "test_data.txt";

const DEBUG: bool = false;
const PART_TWO: bool = true;
const MAGIC_NUMBER: u64 = 19690720;
const MAGIC_NUMBER_DEBUG: u64 = 30;

fn parse_line_to_numbers(line: &str) -> Vec<u64>
{
    let mut numbers: Vec<u64> = Vec::new();

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
                eprintln!("Could not convert '{}' to a u64, error: {}", n, e);
                process::exit(1);
            }
        }
    }

    numbers
}

fn print_program(program: &Vec<u64>)
{
    for p in program
    {
        print!("{}, ", p);
    }
    println!("");
}

fn run_program(program: &mut Vec<u64>)
{
    let mut index: usize = 0;
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

                println!("index1: {}", index1);
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

fn solve_for_values(program: &Vec<u64>, magic_number: u64, noun_bound: (u64, u64), verb_bound: (u64, u64)) -> (u64, u64)
{
    let noun: u64 = (noun_bound.1 + noun_bound.0) / 2; 
    let verb: u64 = (verb_bound.1 + verb_bound.0) / 2; 

    let mut new_program = program.clone();
    new_program[1] = noun;
    new_program[2] = verb;
    run_program(&mut new_program);

    let solution: u64 = new_program[0];
    println!("Noun: {} ({}, {}), verb: {} ({}, {}), solution: {} (MN: {})", noun, noun_bound.0, noun_bound.1, verb, verb_bound.0, verb_bound.1, solution, magic_number);

    if solution == magic_number
    {
        return (noun, verb);
    }
    else if solution < magic_number
    {
        if noun + 1 <= noun_bound.1
        {
            let sub = solve_for_values(program, magic_number, (noun + 1, noun_bound.1), verb_bound);
            if sub != (0, 0)
            {
                return sub;
            }
        }
        if verb + 1 <= verb_bound.1
        {
            let sub = solve_for_values(program, magic_number, noun_bound, (verb + 1, verb_bound.1));

            if sub != (0, 0)
            {
                return sub;
            }
        }
        return (0, 0);
    }
    else // solution > magic_number
    {
        if noun == 0
        {
            return (0, 0);
        }
        if noun_bound.0 <= noun - 1
        {
            let sub = solve_for_values(program, magic_number, (noun_bound.0, noun - 1), verb_bound);
            if sub != (0, 0)
            {
                return sub;
            }
        }
        if verb == 0
        {
            return (0, 0);
        }
        if verb_bound.0 <= verb - 1
        {
            let sub = solve_for_values(program, magic_number, noun_bound, (verb_bound.0, verb - 1));
            if sub != (0, 0)
            {
                return sub;
            }
        }
        
        return (0, 0);
    }
}

fn solve_for_magic_number(program: &mut Vec<u64>, magic_number: u64) -> u64
{
    let mut noun_bound: (u64, u64) = (0, 99);
    let mut verb_bound: (u64, u64) = (0, 99);
    if DEBUG
    {
        if noun_bound.1 > program.len().try_into().unwrap()
        {
            noun_bound.1 = program.len().try_into().unwrap();
            verb_bound.1 = program.len().try_into().unwrap();
        }
    }
    let (noun, verb) = solve_for_values(&program, magic_number, noun_bound, verb_bound);

    100 * noun + verb
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

    let mut program: Vec<u64> = Vec::new(); 

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
    if !PART_TWO
    {
        if !DEBUG
        {
            program[1] = 12;
            program[2] = 2;
        }
        run_program(&mut program);
        let val = program[0];
        println!("val at index 0 is: {}", val);
        println!("The result in postion 0 is: {}", program[0]);
    }
    else
    {
        let result: u64;
        if DEBUG
        {
            result = solve_for_magic_number(&mut program, MAGIC_NUMBER_DEBUG);
        }
        else // not DEBUG
        {
            result = solve_for_magic_number(&mut program, MAGIC_NUMBER);
        }
        println!("The result is: {}", result);
    }

    let duration = start_time.elapsed();
    

    println!("Finished running in: {:.3?}", duration);
}
