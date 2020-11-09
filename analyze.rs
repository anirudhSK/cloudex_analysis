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
    let mut enq_ts_index     = usize::MAX;
    let mut deq_ts_index     = usize::MAX;
    let mut qdelay_ts_index  = usize::MAX; 
    let mut column_name_to_index_map = HashMap::new();
    let mut qdelaysum_vector  : Vec::<i64> = vec![0; NUM_SECONDS]; // one entry for each second.
    let mut qdelaycount_vector: Vec::<i64> = vec![0; NUM_SECONDS]; // one entry for each second.
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
                println!("Print column_name_to_index_map: {:#?}", column_name_to_index_map);
                ts_index = column_name_to_index_map["Genesis-Timestamp"];
                enq_ts_index = column_name_to_index_map["Enqueue-Timestamp"];
                deq_ts_index = column_name_to_index_map["Dequeue-Timestamp"];
                qdelay_ts_index = column_name_to_index_map["3"];
            } else {
                if line.is_ok() {
                    record_count += 1;
                    let tmp = line.unwrap(); 
                    let records = tmp.split(',').collect::<Vec::<&str>>();
                    let now  = i64::from_str(records[ts_index]).unwrap();
                    let orig_qdelay = (f64::from_str(records[qdelay_ts_index]).unwrap() * 1000.0).round() as i64;
                    let qdelay     = i64::from_str(records[deq_ts_index]).unwrap() -
                                     i64::from_str(records[enq_ts_index]).unwrap();
                    qdelaysum_vector[((now-base_time)/1000000) as usize] += qdelay;
                    qdelaycount_vector[((now-base_time)/1000000) as usize] += 1;
                    assert!(orig_qdelay == qdelay);
                    if record_count % 10000 == 0 {
                        println!("Done with {} records", record_count);
                    }
                }
            }
        }
    }
    let mut avg_qdelay = Vec::new();
    for i in 0..NUM_SECONDS {
        if qdelaycount_vector[i] == 0 {
            avg_qdelay.push(-1.0000);
        } else {
            avg_qdelay.push(qdelaysum_vector[i] as f64/qdelaycount_vector[i] as f64);
        }
    }
    println!("Queuing delay: {:#?}", avg_qdelay);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

