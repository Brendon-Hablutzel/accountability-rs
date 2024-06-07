use std::path::Path;

use accountability_rs::stream_activity_records;

fn main() {
    let log_filepath = Path::new("log.json");

    let logs = stream_activity_records(log_filepath).unwrap();

    for log in logs {
        if let Ok(log) = log {
            println!("{}", log.record_date)
        } else {
            println!("error parsing log")
        }
    }
}
