use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::cmp;
use std::i64;
use std::str::FromStr;

fn main() {
    let mut record_count     = 0;
    let mut first_line_done  = false;
    let mut start_time       = i64::MAX;
    let mut end_time         = 0;
    let mut ts_index         = usize::MAX;
    let mut column_name_to_index_map = HashMap::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(env::args().nth(1).unwrap()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if ! first_line_done {
                // Split unwrap from split to avoid "temporary value borrowed while dropped" error
                // https://users.rust-lang.org/t/can-not-understand-temporary-value-dropped-while-borrowed/23279/7
                // https://stackoverflow.com/a/26080489
                let tmp = line.unwrap();
                let column_names = tmp.split(',').collect::<Vec::<&str>>();
                let mut i = 0;
                for column_name in column_names {
                    i = i + 1;
                    column_name_to_index_map.insert(column_name.to_string(), i); 
                }
                first_line_done = true;
                println!("Print column_name_to_index_map: {:#?}", column_name_to_index_map);
                ts_index = column_name_to_index_map["Genesis-Timestamp"];
            } else {
                if line.is_ok() {
                    record_count += 1;
                    let tmp = line.unwrap(); 
                    let records = tmp.split(',').collect::<Vec::<&str>>();
                    start_time = cmp::min(i64::from_str(records[ts_index]).unwrap(), start_time);
                    end_time   = cmp::max(i64::from_str(records[ts_index]).unwrap(), end_time);
                    if record_count % 1000 == 0 {
                        println!("Done with {} records", record_count);
                    }
                }
            }
        }
    }
    println!("Total records: {}, start_time = {}, end_time = {}", record_count, start_time, end_time);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

