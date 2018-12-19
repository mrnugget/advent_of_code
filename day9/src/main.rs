extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn parse_input(input: &str) -> (u32, u32) {
    let line_re: Regex = Regex::new(r"(\d+).*\s(\d+)").unwrap();

    let caps = line_re.captures(input).unwrap();
    let players = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let last = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

    (players, last)
}

#[derive(Debug)]
struct Marble {
    id: usize,
    value: u32,
    next: usize,
    prev: usize,
}

fn calculate_high_score(num_players: u32, last_marble: u32) -> u32 {
    let mut players: Vec<u32> = (0..num_players).map(|_| 0).collect();
    let mut current_player = 0;

    let mut marbles = vec![Marble {
        id: 0,
        value: 0,
        next: 0,
        prev: 0,
    }];
    let mut current_marble_id = 0;

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let mut i = 7;
            while i > 0 {
                current_marble_id = marbles[current_marble_id].prev;
                i -= 1;
            }
            players[current_player] += marbles[current_marble_id].value;
            players[current_player] += marble;

            let prev_id = marbles[current_marble_id].prev;
            let next_id = marbles[current_marble_id].next;
            marbles[prev_id].next = next_id;
            marbles[next_id].prev = prev_id;
            current_marble_id = next_id;
        } else {
            let left_id = marbles[current_marble_id].next;
            let right_id = marbles[left_id].next;

            let new = Marble {
                id: marbles.len(),
                value: marble,
                prev: left_id,
                next: right_id,
            };
            marbles[left_id].next = new.id;
            marbles[right_id].prev = new.id;
            current_marble_id = new.id;
            marbles.push(new);
        }

        current_player = (current_player + 1) % num_players as usize;
    }

    // println!("players={:?}", players);
    *players.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input<'a>() -> &'a str {
        "10 players; last marble is worth 1618 points"
    }

    #[test]
    fn test_parsing_input() {
        let input = test_input();
        assert_eq!(parse_input(input), (10, 1618));
    }

    #[test]
    fn test_calculating_high_score() {
        assert_eq!(calculate_high_score(9, 25), 32);
        assert_eq!(calculate_high_score(10, 1618), 8317);
        assert_eq!(calculate_high_score(13, 7999), 146373);
        assert_eq!(calculate_high_score(17, 1104), 2764);
        assert_eq!(calculate_high_score(21, 6111), 54718);
        assert_eq!(calculate_high_score(30, 5807), 37305);
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

    let first_line = contents.lines().next().unwrap();
    let (players, last_marble_worth) = parse_input(first_line);
    println!(
        "players={}, last_marble_worth={}",
        players, last_marble_worth
    );

    // Part 1
    let high_score = calculate_high_score(players, last_marble_worth);
    println!("high_score={}", high_score);
    assert_eq!(high_score, 410375);

    // Part 2
    let new_high_score = calculate_high_score(players, last_marble_worth * 100);
    println!("new_high_score={}", new_high_score);
    assert_eq!(new_high_score, 3314195047);

    Ok(())
}
