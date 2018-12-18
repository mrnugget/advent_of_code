extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn build_graph<'a, I>(lines: I) -> HashMap<&'a str, Vec<&'a str>>
where
    I: Iterator<Item = &'a str>,
{
    let line_re: Regex = Regex::new(r"Step\s(\w)\smust\sbe\sfinished\sbefore\sstep\s(\w)").unwrap();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let caps = line_re.captures(line).unwrap();
        let parent = caps.get(1).unwrap().as_str();
        let child = caps.get(2).unwrap().as_str();
        graph.entry(parent).or_default();
        graph.entry(child).or_default().push(parent);
    }

    graph
}

fn get_next_steps<'a>(
    steps: &HashSet<&'a str>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    satisfied: &HashSet<&'a str>,
) -> Vec<&'a str> {
    let mut result: Vec<&str> = steps
        .iter()
        .cloned()
        .filter(|step| match graph.get(*step) {
            Some(deps) => deps.iter().all(|dep| satisfied.contains(dep)),
            None => true,
        })
        .collect();
    result.sort();
    result
}

fn build_order(graph: HashMap<&str, Vec<&str>>) -> String {
    let mut steps: HashSet<&str> = graph.keys().cloned().collect();
    let mut result: Vec<&str> = Vec::new();
    let mut satisfied: HashSet<&str> = HashSet::new();

    while !steps.is_empty() {
        let next = get_next_steps(&steps, &graph, &satisfied);
        if let Some(&step) = next.first() {
            satisfied.insert(step);
            result.push(step);
            steps.remove(step);
        }
    }

    result.into_iter().collect::<String>()
}

#[derive(Debug)]
struct Worker<'a> {
    work_left: u32,
    step: Option<&'a str>,
}

fn step_duration(step: &str) -> u32 {
    step.chars().next().unwrap() as u32 - ('A' as u32) + 1
}

fn work_off_steps(graph: HashMap<&str, Vec<&str>>, num_workers: i32, base_duration: u32) -> i32 {
    let mut steps: HashSet<&str> = graph.keys().cloned().collect();
    let steps_len = steps.len();
    let mut worked_off: HashSet<&str> = HashSet::new();

    let mut workers = Vec::new();
    for _ in 0..num_workers {
        workers.push(Worker {
            work_left: 0,
            step: None,
        });
    }

    let mut duration = 0;

    while !steps.is_empty() || worked_off.len() != steps_len {
        let next = get_next_steps(&steps, &graph, &worked_off);

        for (worker, step) in workers.iter_mut().filter(|w| w.step == None).zip(next) {
            worker.work_left = base_duration + step_duration(step);
            worker.step = Some(step);

            steps.remove(step);
        }

        for w in workers.iter_mut().filter(|w| w.step != None) {
            w.work_left -= 1;
            if w.work_left == 0 {
                worked_off.insert(w.step.unwrap());
                w.step = None;
            }
        }

        duration += 1;
    }

    duration
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input<'a>() -> Vec<&'a str> {
        vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]
    }

    #[test]
    fn test_building_graph() {
        let input = test_input();
        let graph = build_graph(input.into_iter());
        let mut keys: Vec<&str> = graph.keys().cloned().collect();
        keys.sort();
        assert_eq!(keys, vec!["A", "B", "C", "D", "E", "F"]);

        assert_eq!(graph[&"C"].len(), 0);
        assert_eq!(graph[&"A"], vec!["C"]);
        assert_eq!(graph[&"B"], vec!["A"]);
        assert_eq!(graph[&"E"], vec!["B", "D", "F"]);
        assert_eq!(graph[&"D"], vec!["A"]);
        assert_eq!(graph[&"F"], vec!["C"]);
    }

    #[test]
    fn test_ordering() {
        let input = test_input();
        let graph = build_graph(input.into_iter());
        let order = build_order(graph);

        assert_eq!(order, "CABDFE");
    }

    #[test]
    fn test_working_off_the_steps() {
        let input = test_input();
        let graph = build_graph(input.into_iter());
        let num_workers = 2;
        let total_duration = work_off_steps(graph.clone(), num_workers, 0);
        assert_eq!(total_duration, 15);
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

    let graph = build_graph(contents.lines());
    let part_1 = build_order(graph.clone());
    assert_eq!(part_1, "BCEFLDMQTXHZGKIASVJYORPUWN");
    println!("part_1={}", part_1);

    let part_2 = work_off_steps(graph.clone(), 5, 60);
    assert_eq!(part_2, 987);
    println!("part_2={}", part_2);
    Ok(())
}
