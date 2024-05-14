use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};

#[derive(Deserialize)]
struct ActivityGoal {
    name: String,
    description: String,
    minimum_minutes: Option<u16>,
    maximum_minutes: Option<u16>,
}

#[derive(Deserialize)]
struct ActivityGoals {
    activity_goals: Vec<ActivityGoal>,
}

#[derive(Serialize)]
struct ActivityRecord {
    short_name: String,
    description: String,
    minutes_spent: u16,
    minimum_minutes: Option<u16>,
    maximum_minutes: Option<u16>,
}

#[derive(Serialize)]
struct ActivitiesRecord {
    timestamp: f64,
    record_date: NaiveDate,
    activities: Vec<ActivityRecord>,
}

impl ActivitiesRecord {
    fn new(timestamp: Duration, record_date: NaiveDate, activities: Vec<ActivityRecord>) -> Self {
        Self {
            timestamp: timestamp.as_secs_f64(),
            record_date,
            activities,
        }
    }
}

fn get_user_input<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    print!("{prompt}: ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    match T::from_str(user_input.trim_end()) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Error, invalid input: {error}");
            get_user_input(prompt)
        }
    }
}

fn load_activity_goals(filepath: &Path) -> ActivityGoals {
    let file = File::open(filepath).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn survey(goals: Vec<ActivityGoal>) -> Vec<ActivityRecord> {
    let mut records = vec![];
    for goal in goals {
        let minutes_spent = get_user_input::<u16>(&format!(
            "How many minutes did you spend on '{}'",
            goal.name
        ));

        let record = ActivityRecord {
            short_name: goal.name,
            description: goal.description,
            minutes_spent,
            minimum_minutes: goal.minimum_minutes,
            maximum_minutes: goal.maximum_minutes,
        };
        records.push(record);
    }

    records
}

fn append_record_to_file(filepath: &Path, record: ActivitiesRecord) {
    let mut file = OpenOptions::new().append(true).open(filepath).unwrap();
    serde_json::to_writer(&mut file, &record).unwrap();
    file.write("\n".as_bytes()).unwrap();
}

fn main() {
    let mut args = env::args();
    args.next(); // skip first argument

    let activity_goals_filepath = args.next().unwrap();
    let activity_goals_filepath = Path::new(&activity_goals_filepath);

    let activity_records_filepath = args.next().unwrap();
    let activity_records_filepath = Path::new(&activity_records_filepath);

    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    println!("Current time since epoch: {:?}", now);

    let activity_goals = load_activity_goals(activity_goals_filepath);

    let date = get_user_input::<NaiveDate>("What date are you recording (yyyy-mm-dd)?");
    println!("Recording for: {date}");

    let activity_records = survey(activity_goals.activity_goals);

    let activities_record = ActivitiesRecord::new(now, date, activity_records);
    append_record_to_file(activity_records_filepath, activities_record);
}
