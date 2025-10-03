use std::fs;
use std::io::BufRead;
use std::process;
use std::time::Instant;

const DATA_FILE_PATH: &str = "data.txt";
const TEST_DATA_FILE_PATH: &str = "test_data.txt";

const DEBUG: bool = false;
const PART_TWO: bool = false;


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

    for result in reader.lines()
    {
        match result
        {
            Ok(line) =>
            {
                // PROCESS THE LINE INPUT HERE
            },
            Err(e) => 
            {
                eprintln!("Error: could not read line from file, {}", e);
                process::exit(1);
            }
        }
    }

    // DO THE PROCESSING OF THE DATA HERE

    let duration = start_time.elapsed();

    println!("Finished running in: {:.3?}", duration);
}
