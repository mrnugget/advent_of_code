#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Guard {
    id: u32,
    minutes_asleep: [u32; 60],
}

#[derive(Debug, PartialEq)]
enum LogType {
    SleepStart,
    SleepEnd,
    ShiftStart(u32),
}

struct Log {
    log_type: LogType,
    time: DateTime<UTC>,
}

fn parse_log_line(line: &str) -> Option<Log> {
    lazy_static! {
        static ref LINE_RE: Regex =
            Regex::new(r"\[(\d+)-(\d+)-(\d+)\s(\d+):(\d+)\]\s(.*)").unwrap();
        static ref SHIFT_START_RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    }
    let caps = LINE_RE.captures(line)?;

    let year = caps.get(1)?.as_str().parse::<i32>().unwrap();
    let month = caps.get(2)?.as_str().parse::<u32>().unwrap();
    let day = caps.get(3)?.as_str().parse::<u32>().unwrap();
    let hour = caps.get(4)?.as_str().parse::<u32>().unwrap();
    let minute = caps.get(5)?.as_str().parse::<u32>().unwrap();
    let message = caps.get(6)?.as_str();

    let time = UTC.ymd(year, month, day).and_hms(hour, minute, 00);
    let log_type = match message {
        "falls asleep" => LogType::SleepStart,
        "wakes up" => LogType::SleepEnd,
        shift_start => {
            let caps = SHIFT_START_RE.captures(shift_start)?;
            let guard_id = caps.get(1)?.as_str().parse::<u32>().unwrap();
            LogType::ShiftStart(guard_id)
        }
    };

    Some(Log { log_type, time })
}

fn parse_logs<'a, I>(lines: I) -> Vec<Log>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut parsed = Vec::new();

    for (_, line) in lines.into_iter().enumerate() {
        if let Some(parsed_line) = parse_log_line(line) {
            parsed.push(parsed_line);
        }
    }

    parsed.sort_unstable_by_key(|k| k.time);
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing() {
        let lines = vec![
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
        ];

        let parsed = parse_log_line(lines[0]).unwrap();
        assert_eq!(parsed.log_type, LogType::ShiftStart(99));
        assert_eq!(parsed.time, UTC.ymd(1518, 11, 01).and_hms(23, 58, 00));

        let parsed = parse_log_line(lines[1]).unwrap();
        assert_eq!(parsed.log_type, LogType::SleepStart);
        assert_eq!(parsed.time, UTC.ymd(1518, 11, 02).and_hms(00, 40, 00));

        let parsed = parse_log_line(lines[2]).unwrap();
        assert_eq!(parsed.log_type, LogType::SleepEnd);
        assert_eq!(parsed.time, UTC.ymd(1518, 11, 02).and_hms(00, 50, 00));
    }

    #[test]
    fn multiple_lines_parsing_and_sorting() {
        let lines = vec![
            "[1518-10-15 00:42] wakes up",
            "[1518-06-11 00:52] falls asleep",
            "[1518-06-22 00:49] wakes up",
            "[1518-06-27 23:58] Guard #2389 begins shift",
            "[1518-10-14 00:11] falls asleep",
            "[1518-03-28 23:59] Guard #1777 begins shift",
            "[1518-06-16 00:46] falls asleep",
        ];

        let logs = parse_logs(lines);
        assert_eq!(logs.len(), 7);
        assert_eq!(logs[0].log_type, LogType::ShiftStart(1777));
        assert_eq!(logs[6].log_type, LogType::SleepEnd);
    }

    #[test]
    fn logs_to_guards() {
        let lines = vec![
            "[1518-03-27 00:03] Guard #2251 begins shift",
            "[1518-03-27 00:11] falls asleep",
            "[1518-03-27 00:57] wakes up",
            "[1518-03-27 23:58] Guard #3319 begins shift",
            "[1518-03-28 00:16] falls asleep",
            "[1518-03-28 00:33] wakes up",
            "[1518-03-28 00:53] falls asleep",
            "[1518-03-28 00:54] wakes up",
            "[1518-03-28 23:59] Guard #1777 begins shift",
            "[1518-03-29 00:08] falls asleep",
            "[1518-03-29 00:10] wakes up",
            "[1518-03-29 00:20] falls asleep",
            "[1518-03-29 00:55] wakes up",
        ];

        let logs = parse_logs(lines);
        assert_eq!(logs.len(), 13);
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("not enough arguments");
        process::exit(1);
    }

    let filename = args[1].clone();
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let logs = parse_logs(contents.lines());

    Ok(())
}
