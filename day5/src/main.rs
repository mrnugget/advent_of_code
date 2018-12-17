use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn units_can_react(a: char, b: char) -> bool {
    a != b && a.to_lowercase().to_string() == b.to_lowercase().to_string()
}

fn react_all(units: &mut Vec<char>) {
    let mut len = units.len();
    let mut i = 1;

    while i < len {
        if units_can_react(units[i - 1], units[i]) {
            units.remove(i - 1);
            units.remove(i - 1);
            if i > 1 {
                i -= 1;
            }
            len -= 2;
            continue;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_all_reacting_units() {
        let mut input: Vec<char> = "dabAcCaCBAcCcaDA".chars().collect();
        let expected: Vec<char> = "dabCBAcaDA".chars().collect();

        react_all(&mut input);
        assert_eq!(input, expected);
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
    units.pop(); // Remove `\n`
    react_all(&mut units);

    println!("part 1 - units left: {}", units.len());

    println!("part 2");

    let mut shortest = units.len();
    for c in b'a'..=b'z' {
        let unit1 = c as char;
        let unit2 = unit1.to_uppercase().into_iter().next().unwrap();

        let mut filtered = units
            .clone()
            .iter()
            .filter(|&&u| u != unit1 && u != unit2)
            .cloned()
            .collect();

        react_all(&mut filtered);

        let len = filtered.len();
        if len < shortest {
            shortest = len;
        }
    }
    println!("shortest: {}", shortest);

    Ok(())
}
