use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn units_can_react(a: char, b: char) -> bool {
    a != b && a.to_lowercase().to_string() == b.to_lowercase().to_string()
}

fn remove_first_reacting_units(units: Vec<char>) -> (Vec<char>, bool) {
    let mut result = Vec::new();
    let mut units_iter = units.iter().peekable();
    let mut units_removed = false;

    while let Some(&current) = units_iter.next() {
        let cloned = current.clone();

        if let Some(&&peek) = units_iter.peek() {
            if !units_removed && units_can_react(current, peek) {
                units_iter.next();
                units_removed = true;
            } else {
                result.push(cloned);
            }
        } else {
            result.push(cloned);
        }
    }

    (result, units_removed)
}

fn remove_all_reacting_units(units: Vec<char>) -> Vec<char> {
    let mut result = units;
    let mut units_removed = true;

    while units_removed {
        let (res, rem) = remove_first_reacting_units(result);
        result = res;
        units_removed = rem;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_no_units() {
        let input: Vec<char> = "aaaaa".chars().collect();

        let (_, units_removed) = remove_first_reacting_units(input);
        assert!(!units_removed);
    }

    #[test]
    fn test_remove_first_reacting_units() {
        let input: Vec<char> = "dabAcCaCBAcCcaDA".chars().collect();
        let expected: Vec<char> = "dabAaCBAcCcaDA".chars().collect();

        let (result, units_removed) = remove_first_reacting_units(input);
        assert!(units_removed);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_all_reacting_units() {
        let input: Vec<char> = "dabAcCaCBAcCcaDA".chars().collect();
        let expected: Vec<char> = "dabCBAcaDA".chars().collect();

        let result = remove_all_reacting_units(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_units_can_react() {
        assert!(units_can_react('A', 'a'));
        assert!(units_can_react('b', 'B'));
        assert!(!units_can_react('b', 'b'));
        assert!(!units_can_react('a', 'B'));
        assert!(!units_can_react('b', 'A'));
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

    let mut units: Vec<char> = contents.chars().collect();
    units.remove(units.len() - 1);
    let result = remove_all_reacting_units(units);

    println!("units left: {}", result.len());

    Ok(())
}
