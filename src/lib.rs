use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

#[derive(Deserialize)]
pub struct ActivityGoal {
    pub name: String,
    pub description: String,
    pub minimum_minutes: Option<u16>,
    pub maximum_minutes: Option<u16>,
}

#[derive(Deserialize)]
pub struct ActivityGoals {
    pub activity_goals: Vec<ActivityGoal>,
}

#[derive(Serialize, Deserialize)]
pub struct ActivityRecord {
    name: String,
    description: String,
    minutes_spent: u16,
    minimum_minutes: Option<u16>,
    maximum_minutes: Option<u16>,
}

impl ActivityRecord {
    pub fn from_goal(goal: ActivityGoal, minutes_spent: u16) -> Self {
        Self {
            name: goal.name,
            description: goal.description,
            minutes_spent,
            minimum_minutes: goal.minimum_minutes,
            maximum_minutes: goal.maximum_minutes,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ActivitiesRecord {
    pub timestamp: f64,
    pub record_date: NaiveDate,
    pub activities: Vec<ActivityRecord>,
}

impl ActivitiesRecord {
    pub fn new(
        timestamp: Duration,
        record_date: NaiveDate,
        activities: Vec<ActivityRecord>,
    ) -> Self {
        Self {
            timestamp: timestamp.as_secs_f64(),
            record_date,
            activities,
        }
    }
}

pub fn load_activity_goals(filepath: &Path) -> Result<ActivityGoals> {
    let file = File::open(filepath)
        .map_err(|err| anyhow!("unable to open goals file {}: {}", filepath.display(), err))?;

    let goals = serde_json::from_reader(file)
        .map_err(|err| anyhow!("unable to parse goals file {}: {}", filepath.display(), err))?;
    Ok(goals)
}

pub fn parse_line(line: String) -> Result<ActivitiesRecord> {
    Ok(serde_json::from_str(&line)?)
}

pub fn stream_activity_records(
    filepath: &Path,
) -> Result<impl Iterator<Item = Result<ActivitiesRecord>>> {
    let file = File::open(filepath).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let lines = lines.map(|line| {
        line.map_err(|err| anyhow!(err))
            .and_then(|line| parse_line(line))
    });

    Ok(lines)
}
