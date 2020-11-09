use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::i64;
use std::str::FromStr;
const NUM_SECONDS : usize      = 10000;

fn main() {
    let mut record_count     = 0;
    let mut first_line_done  = false;
    let mut ts_index         = usize::MAX;
    let mut qdelay_ts_index  = usize::MAX; 
    let mut column_name_to_index_map = HashMap::new();
    let base_time = i64::from_str(&env::args().nth(2).unwrap()).unwrap();
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
                    column_name_to_index_map.insert(column_name.to_string(), i);
                    i = i + 1;
                }
                first_line_done = true;
                ts_index = column_name_to_index_map["Genesis-Timestamp"];
                qdelay_ts_index = column_name_to_index_map["3"];
            } else {
                if line.is_ok() {
                    record_count += 1;
                    let tmp = line.unwrap(); 
                    let records = tmp.split(',').collect::<Vec::<&str>>();
                    let now  = i64::from_str(records[ts_index]).unwrap();
                    let orig_qdelay = (f64::from_str(records[qdelay_ts_index]).unwrap() * 1000.0).round() as i64;
                    println!("{} {}", now - base_time, orig_qdelay);
                    if record_count % 10000 == 0 {
                        eprintln!("Done with {} records", record_count);
                    }
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

