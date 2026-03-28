use std::fmt::Display;
use std::fs;
use std::io::BufRead;
use std::process::{self, exit};
use std::time::Instant;

use clap::Parser;

const DATA_FILE_PATH: &str = "data.txt";
const TEST_DATA_FILE_PATH: &str = "test_data.txt";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args
{
    #[arg(short, long, default_value_t=false)]
    debug: bool,

    #[arg(short, long, default_value_t=false)]
    part_two: bool,
}

fn parse_line_to_numbers(line: &str) -> Vec<i64>
{
    let mut numbers: Vec<i64> = Vec::new();

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
                eprintln!("Could not convert '{}' to a i64, error: {}", n, e);
                process::exit(1);
            }
        }
    }

    numbers
}

#[repr(i64)]
enum Opcode
{
    ADD = 1,
    MUL = 2,
    INPUT = 3,
    OUTPUT = 4,
    JMP_EQ = 5,
    JMP_NE = 6,
    LESS = 7,
    EQUAL = 8,
    EXIT = 99,
}

impl Display for Opcode
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let text = match self
        {
            Opcode::ADD => "ADD",
            Opcode::MUL => "MUL",
            Opcode::INPUT => "INPUT",
            Opcode::OUTPUT => "OUPUT",
            Opcode::JMP_EQ => "JUMP_EQ",
            Opcode::JMP_NE => "JUMP_NE",
            Opcode::LESS => "LESS",
            Opcode::EQUAL => "EQUAL",
            Opcode::EXIT => "EXIT"
        };
        write!(f, "{}", text)
    }
}

#[repr(i64)]
enum Mode
{
    POSITION = 0,
    IMMEDIATE = 1,
}

impl Display for Mode
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let text = match self
        {
            Mode::POSITION => "POSITION",
            Mode::IMMEDIATE => "IMMEDIATE"
        };

        write!(f, "{}", text)
    }
}

struct Instruction
{
    opcode: Opcode,
    first_param: Mode,
    second_param: Mode,
    third_param: Mode,
}

impl Instruction
{
    pub fn new(value: i64) -> Self
    {
        let mut ins = value;
        let opcode = match ins % 100
        {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            3 => Opcode::INPUT,
            4 => Opcode::OUTPUT,
            5 => Opcode::JMP_EQ,
            6 => Opcode::JMP_NE,
            7 => Opcode::LESS,
            8 => Opcode::EQUAL,
            99 => Opcode::EXIT,
            other => {
                eprintln!("Invalid opcode: {}", other);
                exit(1);
            }
            
        };
        ins = ins / 100;

        let first_param = match ins % 10
        {
            0 => Mode::POSITION,
            1 => Mode::IMMEDIATE,
            _ => {
                eprintln!("Could not extract first param mode, {}", ins % 10);
                exit(1);
            }
        };

        ins = ins / 10;
        let second_param = match ins % 10
        {
            0 => Mode::POSITION,
            1 => Mode::IMMEDIATE,
            _ => exit(1)
        };


        ins = ins / 10;
        let third_param = match ins % 10
        {
            0 => Mode::POSITION,
            1 => Mode::IMMEDIATE,
            _ => exit(1)
        };

        Instruction { opcode, first_param, second_param, third_param }
    }

    pub fn print(&self)
    {
        println!("INSTRUCTION: opcode: {}, f_p: {}, s_p: {}, t_p: {}", 
           
            self.opcode, self.first_param, self.second_param, self.third_param);
    }
}

fn run_program(program: &mut Vec<i64>, input: i64) -> Vec<i64>
{
    let mut index: usize = 0;
    let mut output: Vec<i64> = vec![];
    loop
    {
        println!("INDEX: {}", index);
        let instruction = Instruction::new(program[index]);
        instruction.print();
        match instruction.opcode
        {
            Opcode::ADD => // Addition
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];
                let index3 = program[index + 3];

                let val1 = match instruction.first_param
                {
                    Mode::POSITION => program[index1 as usize],
                    Mode::IMMEDIATE => index1,
                };
                let val2 = match instruction.second_param
                {
                    Mode::POSITION => program[index2 as usize],
                    Mode::IMMEDIATE => index2,
                };
                let result = val1 + val2;
                program[index3 as usize] = result;

                println!("ADD {}({}) {}({}) => {}({})", val1, index1, val2, index2, result, index3);
                index += 4;
            }
            Opcode::MUL => // Multiplication
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];
                let index3 = program[index + 3];

                let val1 = match instruction.first_param
                {
                    Mode::POSITION => program[index1 as usize],
                    Mode::IMMEDIATE => index1,
                };
                let val2 = match instruction.second_param
                {
                    Mode::POSITION => program[index2 as usize],
                    Mode::IMMEDIATE => index2,
                };

                let result = val1 * val2;
                program[index3 as usize] = result;

                println!("MUL {}({}) {}({}) => {}({})", val1, index1, val2, index2, result, index3);
                index += 4;
            }
            Opcode::INPUT => 
            {
                let index1 = program[index + 1];
                program[index1 as usize] = input;

                println!("INPUT {} => [{}]", input, index1);
                index += 2;
            }
            Opcode::OUTPUT => 
            {
                let index1 = program[index + 1];


                let val1 = match instruction.first_param
                {
                    Mode::POSITION => program[index1 as usize],
                    Mode::IMMEDIATE => index1,
                };
                output.push(val1);

                println!("OUTPUT [{}] => {}", index1, val1);
                index += 2
            }
            Opcode::JMP_EQ => 
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];
                
                let val1 = match instruction.first_param
                {
                    Mode::POSITION => program[index1 as usize],
                    Mode::IMMEDIATE => index1,
                };

                if val1 != 0 as i64
                {
                    let val2 = match instruction.second_param
                    {
                        Mode::POSITION => program[index2 as usize],
                        Mode::IMMEDIATE => index2,
                    };
                    index = val2 as usize;
                    println!("JMP_EQ (YES) {} => [{}]", val1, val2);
                }
                else
                {
                    index += 3;
                    println!("JMP_EQ (NO) {} => [{}]", val1, index);
                }
            }
            Opcode::JMP_NE => 
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];

                let val1 = match instruction.first_param
                {
                    Mode::POSITION => program[index1 as usize],
                    Mode::IMMEDIATE => index1,
                };

                if val1 == 0 as i64
                {
                    let val2 = match instruction.second_param
                    {
                        Mode::POSITION => program[index2 as usize],
                        Mode::IMMEDIATE => index2,
                    };
                    index = val2 as usize;
                    println!("JMP_NE (YES) {} => [{}]", val1, val2);
                }
                else
                {
                    index += 3;
                    println!("JMP_NE (NO) {} => [{}]", val1, index);
                }
                
            }
            Opcode::LESS => 
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];
                let index3 = program[index + 3];

                let val1 = match instruction.first_param
                {
                    Mode::POSITION => program[index1 as usize],
                    Mode::IMMEDIATE => index1,
                };
                let val2 = match instruction.second_param
                {
                    Mode::POSITION => program[index2 as usize],
                    Mode::IMMEDIATE => index2,
                };

                let mut store_value = 0;

                if val1 < val2
                {
                    store_value = 1;
                }

                program[index3 as usize] = store_value;

                index += 4;
                println!("LESS {}({}) {}({}) => {}({})", val1, index1, val2, index2, store_value, index3);
            }
            Opcode::EQUAL => 
            {
                let index1 = program[index + 1];
                let index2 = program[index + 2];
                let index3 = program[index + 3];

                let val1 = match instruction.first_param
                {
                    Mode::POSITION => program[index1 as usize],
                    Mode::IMMEDIATE => index1,
                };
                let val2 = match instruction.second_param
                {
                    Mode::POSITION => program[index2 as usize],
                    Mode::IMMEDIATE => index2,
                };

                let mut store_value = 0;

                if val1 == val2
                {
                    store_value = 1;
                }

                program[index3 as usize] = store_value;

                index += 4;

                println!("EQUAL {}({}) {}({}) => {}({})", val1, index1, val2, index2, store_value, index3);
            }
            Opcode::EXIT => // Exit program
            {
                println!("EXIT");
                break;
            }
        }
    }

    output
}


fn main()
{
    let args = Args::parse();

    println!("Starting...");
    let filename = if args.debug { TEST_DATA_FILE_PATH } else { DATA_FILE_PATH };
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

    let mut program: Vec<i64> = vec![]; 

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

    if !args.part_two
    {
        let mut outputs = run_program(&mut program, 5);
       
        for v in &outputs
        {
            println!("OUT: {}", v);
        }

        let diagnostic_code = outputs.pop();
        for val in &outputs
        {
            if *val != 0
            {
                eprintln!("Found non-zero report, {}", val);
                exit(1);
            }
        }
        println!("Diagnostic code: {}", diagnostic_code.unwrap());
    }
    else
    {
        println!("PART 2");
    }

    let duration = start_time.elapsed();

    println!("Finished running in: {:.3?}", duration);
}
