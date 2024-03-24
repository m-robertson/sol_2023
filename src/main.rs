use std::env;
use std::fs::File;
use std::io::Read;

mod day_1;

// Entry point
fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure correct number of arguments
    if args.len() != 3 {
        println!("Usage: {} <day_number> <input_file_path>", args[0]);
        return;
    }

    // Parse arguments
    let module_number: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Day number must be an integer");
            return;
        }
    };

    let file_path = &args[2];

    // Open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file");
            return;
        }
    };

    // Read file content
    let mut content = String::new();
    if let Err(_) = file.read_to_string(&mut content) {
        println!("Failed to read file");
        return;
    }

    // Choose the appropriate module based on the module number
    match module_number {
        1 => day_1::process_file(&content),
        _ => {
            println!("Invalid day");
            return;
        }
    };
}
