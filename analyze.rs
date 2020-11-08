use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;

fn main() {
    let mut record_count     = 0;
    let mut first_line_done  = false;
    let mut column_name_to_index_map = HashMap::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(env::args().nth(1).unwrap()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if line.is_ok() {
                record_count += 1;
            }
            if ! first_line_done {
                // Split unwrap from split to avoid temporary value borrowed while dropped
                // https://users.rust-lang.org/t/can-not-understand-temporary-value-dropped-while-borrowed/23279/7
                let tmp = line.unwrap();
                let column_names = tmp.split(',').collect::<Vec::<&str>>();
                let mut i = 0;
                for column_name in column_names {
                    i = i + 1;
                    column_name_to_index_map.insert(column_name.to_string(), i); 
                }
                first_line_done = true;
                println!("Print column_name_to_index_map: {:#?}", column_name_to_index_map);
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

