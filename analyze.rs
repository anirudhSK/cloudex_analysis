use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let mut record_count    = 0;
    let mut first_line_done = false;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(env::args().nth(1).unwrap()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if line.is_ok() {
                record_count += 1;
            }
            if ! first_line_done {
                println!("All column names: {:#?}", line.unwrap().split(',').collect::<Vec::<&str>>());
                first_line_done = true;
            }
        }
    }
    println!("Total records: {}", record_count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

