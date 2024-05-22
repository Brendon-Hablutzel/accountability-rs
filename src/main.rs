use accountability_rs::{load_activity_goals, ActivitiesRecord, ActivityGoal, ActivityRecord};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use std::{
    env,
    fmt::Display,
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
    str::FromStr,
    time::UNIX_EPOCH,
};

fn get_user_input<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    print!("{prompt}: ");
    io::stdout()
        .flush()
        .expect("should be able to flush stdout");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("should be able to read from stdin");

    match T::from_str(user_input.trim_end()) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Error, invalid input: {error}");
            get_user_input(prompt)
        }
    }
}

fn survey(goals: Vec<ActivityGoal>) -> Vec<ActivityRecord> {
    let mut records = vec![];
    for goal in goals {
        let minutes_spent = get_user_input::<u16>(&format!(
            "How many minutes did you spend on '{}'",
            goal.name
        ));

        let record = ActivityRecord::from_goal(goal, minutes_spent);
        records.push(record);
    }

    records
}

fn append_record_to_file(filepath: &Path, record: ActivitiesRecord) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filepath)
        .map_err(|err| anyhow!("unable to open record file {}: {}", filepath.display(), err))?;

    serde_json::to_writer(&mut file, &record).map_err(|err| {
        anyhow!(
            "error writing activity record to {}: {}",
            filepath.display(),
            err
        )
    })?;

    file.write("\n".as_bytes()).map_err(|err| {
        anyhow!(
            "error writing newline to record file {}: {}",
            filepath.display(),
            err
        )
    })?;

    Ok(())
}

fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // skip first argument

    let activity_goals_filepath = args.next().ok_or(anyhow!(
        "a filepath for the activity goals file is required as the first command line argument"
    ))?;
    let activity_goals_filepath = Path::new(&activity_goals_filepath);

    let activity_records_filepath = args.next().ok_or(anyhow!(
        "a filepath for the activity record logs file is required as the second command line argument"
    ))?;
    let activity_records_filepath = Path::new(&activity_records_filepath);

    println!("Reading goals from '{}'", activity_goals_filepath.display());
    println!("Writing log to '{}'", activity_records_filepath.display());

    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("current time should be after unix epoch");
    println!("Current time since epoch: {:?}", now);

    let activity_goals = load_activity_goals(activity_goals_filepath)?;

    let date = get_user_input::<NaiveDate>("What date are you recording (yyyy-mm-dd)?");
    println!("Recording for: {date}");

    let activity_records = survey(activity_goals.activity_goals);

    let activities_record = ActivitiesRecord::new(now, date, activity_records);
    append_record_to_file(activity_records_filepath, activities_record)?;

    Ok(())
}
