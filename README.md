# Accountability

A simple CLI program to track progress toward daily goals. Simply provide a goals file to read from and a log file to write to, then each day, just run `cargo run /path/to/goals.json /path/to/log.json` and enter your answers in the interactive survey. All data will be stored locally in the log file.

## Goals File

The format for the goals file is as follows:

```json
{
    {
        "activity_goals": [
            {
                "name": "activity_1",
                "description": "Some activity with a minimum minutes requirement",
                "minimum_minutes": 45
            },
            {
                "name": "activity_2",
                "description": "Some other activity with a maximum minutes requirement",
                "maximum_minutes": 30
            },
            {
                "name": "activity_3",
                "description": "A final activity with no requirements"
            }
        ]
    }
}
```
