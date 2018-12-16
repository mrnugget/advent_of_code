#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Guard {
    id: u32,
    minutes_asleep: [u32; 60],
}

impl Guard {
    fn new(id: u32) -> Guard {
        Guard {
            id: id,
            minutes_asleep: [0; 60],
        }
    }

    fn sum_minutes_asleep(&self) -> u32 {
        self.minutes_asleep.iter().fold(0, |a, &b| a + b)
    }

    fn sleepiest_minute(&self) -> usize {
        let mut i = 0;

        for (j, &value) in self.minutes_asleep.iter().enumerate() {
            if value > self.minutes_asleep[i] {
                i = j;
            }
        }

        i
    }

    fn highest_sleep(&self) -> u32 {
        *self.minutes_asleep.iter().max_by_key(|x| *x).unwrap()
    }
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

fn logs_to_guards(logs: Vec<Log>) -> HashMap<u32, Guard> {
    let mut guards = HashMap::new();

    let mut current_id = 0;
    let mut sleep_start = 0;

    for (_, log) in logs.iter().enumerate() {
        match log.log_type {
            LogType::ShiftStart(id) => {
                guards.entry(id).or_insert_with(|| Guard::new(id));
                current_id = id;
            }
            LogType::SleepStart => {
                sleep_start = log.time.minute() as usize;
            }
            LogType::SleepEnd => {
                let sleep_end = log.time.minute() as usize;

                let mut guard = guards.get_mut(&current_id).unwrap();

                for minute in sleep_start..sleep_end {
                    guard.minutes_asleep[minute] += 1;
                }
            }
        }
    }

    guards
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
    fn converting_logs_to_guards() {
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
            "[1518-03-30 00:30] Guard #2251 begins shift",
            "[1518-03-30 00:40] falls asleep",
            "[1518-03-30 00:55] wakes up",
        ];

        let logs = parse_logs(lines);
        assert_eq!(logs.len(), 16);

        let guards = logs_to_guards(logs);
        let guard_1 = guards.get(&2251).unwrap();
        assert!(guard_1.minutes_asleep[0..10].iter().all(|m| *m == 0));
        assert!(guard_1.minutes_asleep[11..39].iter().all(|m| *m == 1));
        assert!(guard_1.minutes_asleep[40..55].iter().all(|m| *m == 2));
        assert!(guard_1.minutes_asleep[56..57].iter().all(|m| *m == 1));
        assert!(guard_1.minutes_asleep[58..59].iter().all(|m| *m == 0));

        let guard_2 = guards.get(&3319).unwrap();
        assert!(guard_2.minutes_asleep[0..15].iter().all(|m| *m == 0));
        assert!(guard_2.minutes_asleep[16..33].iter().all(|m| *m == 1));
        assert!(guard_2.minutes_asleep[34..52].iter().all(|m| *m == 0));
        assert!(guard_2.minutes_asleep[53..54].iter().all(|m| *m == 1));
        assert!(guard_2.minutes_asleep[55..59].iter().all(|m| *m == 0));

        let guard_3 = guards.get(&1777).unwrap();
        assert!(guard_3.minutes_asleep[0..7].iter().all(|m| *m == 0));
        assert!(guard_3.minutes_asleep[08..10].iter().all(|m| *m == 1));
        assert!(guard_3.minutes_asleep[11..19].iter().all(|m| *m == 0));
        assert!(guard_3.minutes_asleep[20..55].iter().all(|m| *m == 1));
        assert!(guard_3.minutes_asleep[56..59].iter().all(|m| *m == 0));
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

    let mut guards = logs_to_guards(logs)
        .into_iter()
        .fold(Vec::new(), |mut acc, kv| {
            let (_, guard) = kv;
            acc.push(guard);
            acc
        });

    for (_, guard) in guards.iter().enumerate() {
        println!(
            "guard #{}.\tminutes_asleep: {:?}",
            guard.id,
            &guard.minutes_asleep[..]
        );
    }

    guards.sort_unstable_by_key(|g| g.sum_minutes_asleep());
    let slept_the_most = guards.last().unwrap();
    println!(
        "guard {} slept the most with {} minutes",
        slept_the_most.id,
        slept_the_most.sum_minutes_asleep()
    );

    println!(
        "sleepiest minute for guard {} was minute {}. result for part 1: {}",
        slept_the_most.id,
        slept_the_most.sleepiest_minute(),
        slept_the_most.id * slept_the_most.sleepiest_minute() as u32
    );

    guards.sort_unstable_by_key(|g| g.highest_sleep());
    let has_sleepiest_minute = guards.last().unwrap();

    println!(
        "guard {} has sleepiest minute {} with {} overlaps. result for part 2: {}",
        has_sleepiest_minute.id,
        has_sleepiest_minute.sleepiest_minute(),
        has_sleepiest_minute.highest_sleep(),
        has_sleepiest_minute.id * has_sleepiest_minute.sleepiest_minute() as u32
    );
    Ok(())
}
