extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn parse_input<'a, I>(lines: I) -> (State, Vec<Rule>)
where
    I: Iterator<Item = &'a str>,
{
    let initial_state_re: Regex = Regex::new(r"initial\sstate:\s(\W+)").unwrap();
    let rules_re: Regex = Regex::new(r"(\W+)\s=>\s(\W)").unwrap();

    let mut initial_state = State { pots: Vec::new() };
    let mut rules = Vec::new();

    for line in lines {
        if let Some(caps) = initial_state_re.captures(line) {
            for c in caps.get(1).unwrap().as_str().chars() {
                initial_state.pots.push(c);
            }
        }

        if let Some(caps) = rules_re.captures(line) {
            let pattern_chars = caps.get(1).unwrap().as_str().chars().collect::<Vec<char>>();
            let pattern = Pattern(
                pattern_chars[0],
                pattern_chars[1],
                pattern_chars[2],
                pattern_chars[3],
                pattern_chars[4],
            );
            let result_chars = caps.get(2).unwrap().as_str().chars().collect::<Vec<char>>();
            rules.push(Rule {
                pattern,
                result: result_chars[0],
            });
        }
    }

    (initial_state, rules)
}

#[derive(Debug)]
struct State {
    pots: Vec<char>,
}

impl State {
    fn from_str(input: &str) -> State {
        let pots = input.chars().collect::<Vec<char>>();
        State { pots }
    }

    fn to_string(&self) -> String {
        self.pots.iter().collect::<String>()
    }

    fn get_pattern_at(&self, pos: i32) -> Pattern {
        let max_pos = (self.pots.len() - 1) as i32;
        let mut pots = ['.'; 5];
        let mut i = 0;

        for j in pos - 2..=pos + 2 {
            if j < 0 || j > max_pos {
                i += 1;
                continue;
            }

            if let Some(&pot) = self.pots.get(j as usize) {
                pots[i] = pot;
            }
            i += 1;
        }

        Pattern(pots[0], pots[1], pots[2], pots[3], pots[4])
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pattern(char, char, char, char, char);

impl Pattern {
    fn from_str(input: &str) -> Pattern {
        let chars = input.chars().collect::<Vec<char>>();
        Pattern::from_vec(chars)
    }

    fn from_vec(input: Vec<char>) -> Pattern {
        Pattern(input[0], input[1], input[2], input[3], input[4])
    }
}

#[derive(Debug)]
struct Rule {
    pattern: Pattern,
    result: char,
}

#[derive(Debug)]
struct Game {
    offset: i32,
    state: State,
    rules: Vec<Rule>,
}

impl Game {
    fn evolve(&mut self) {
        let mut new_pots = self.state.pots.clone();
        let len = self.state.pots.len() as i32;
        let start_pos = -2 as i32;
        let end_pos = len + 2;

        let mut prepend = ['.', '.'];
        let mut append = ['.', '.'];

        for i in start_pos..end_pos {
            let pattern = self.state.get_pattern_at(i);

            for rule in &self.rules {
                if rule.pattern == pattern {
                    if i < 0 {
                        prepend[(i + 2) as usize] = rule.result;
                        continue;
                    }
                    if i > len - 1 {
                        append[(i - len) as usize] = rule.result;
                        continue;
                    }
                    new_pots[i as usize] = rule.result;
                }
            }
        }

        if prepend[0] == '#' {
            self.offset = -2;
            new_pots.insert(0, prepend[1]);
            new_pots.insert(0, prepend[0]);
        } else if prepend[1] == '#' {
            self.offset = -1;
            new_pots.insert(0, prepend[1]);
        }

        if append[1] == '#' {
            for &c in append.iter() {
                new_pots.push(c);
            }
        } else if append[0] == '#' {
            new_pots.push(append[0]);
        }

        self.state.pots = new_pots;
    }

    fn sum(&self) -> i32 {
        let mut sum = 0;
        for (i, &p) in self.state.pots.iter().enumerate() {
            if p == '#' {
                sum += (i as i32) + self.offset;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input<'a>() -> Vec<&'a str> {
        vec![
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]
    }

    #[test]
    fn test_parsing_input() {
        let input = test_input();
        let (initial_state, rules) = parse_input(input.into_iter());
        assert_eq!(initial_state.to_string(), "#..#.#..##......###...###",);
        assert_eq!(rules.len(), 14);
        assert_eq!(rules[0].result, '#');
        assert_eq!(rules[0].pattern, Pattern('.', '.', '.', '#', '#'));
    }

    #[test]
    fn test_building_pattern_per_pot() {
        let state = State::from_str("#..#.#..##......###...###");
        let len = state.pots.len() as i32;

        assert_eq!(state.get_pattern_at(-2), Pattern::from_str("....#"));
        assert_eq!(state.get_pattern_at(-1), Pattern::from_str("...#."));
        assert_eq!(state.get_pattern_at(0), Pattern::from_str("..#.."));
        assert_eq!(state.get_pattern_at(1), Pattern::from_str(".#..#"));
        assert_eq!(state.get_pattern_at(len - 2), Pattern::from_str(".###."));
        assert_eq!(state.get_pattern_at(len - 1), Pattern::from_str("###.."));
        assert_eq!(state.get_pattern_at(len), Pattern::from_str("##..."));
        assert_eq!(state.get_pattern_at(len + 1), Pattern::from_str("#...."));
        assert_eq!(state.get_pattern_at(len + 2), Pattern::from_str("....."));
    }

    #[test]
    fn test_evolving_a_game_without_spread() {
        let input = test_input();
        let (_, rules) = parse_input(input.into_iter());
        let mut game = Game {
            offset: 0,
            state: State::from_str("...##"),
            rules,
        };

        assert_eq!(game.state.to_string(), "...##");
        game.evolve();
        assert_eq!(game.state.to_string(), "..###",);
    }

    #[test]
    fn test_evolving_a_game_with_spread() {
        let input = test_input();
        let (_, rules) = parse_input(input.into_iter());
        let mut game = Game {
            offset: 0,
            state: State::from_str("##..."),
            rules,
        };

        assert_eq!(game.state.to_string().len(), 5);
        assert_eq!(game.state.to_string(), "##...");
        game.evolve();
        assert_eq!(game.offset, -1);
        assert_eq!(game.state.to_string().len(), 6);
        assert_eq!(game.state.to_string(), "###...",);
    }

    #[test]
    fn test_game_sum() {
        let state = State::from_str("#....##....#####...#######....#.#..##");
        let rules = Vec::new();
        let offset = -2;

        let game = Game {
            offset,
            rules,
            state,
        };

        assert_eq!(game.sum(), 325);
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

    let (initial_state, rules) = parse_input(contents.lines());

    let mut game = Game {
        offset: 0,
        state: initial_state,
        rules: rules,
    };
    println!("0: {}", game.state.to_string());
    for i in 1..=20 {
        game.evolve();
        println!("{}: {}", i, game.state.to_string());
    }

    // Part 1
    let sum_after_tweny_generations = game.sum();
    println!("sum={}", sum_after_tweny_generations);
    assert_eq!(sum_after_tweny_generations, 1623);

    Ok(())
}
