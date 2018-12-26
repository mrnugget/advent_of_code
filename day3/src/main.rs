extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

type Point = (u32, u32);
type Canvas = HashMap<Point, u32>;

#[derive(Debug)]
struct Claim {
    id: u32,
    start_row: u32,
    start_column: u32,
    width: u32,
    height: u32,
}

impl Claim {
    pub fn new(claim_def: &str) -> Claim {
        let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
        let caps = re.captures(claim_def).unwrap();

        let results: Vec<u32> = (1..=5)
            .map(|i| caps.get(i).unwrap())
            .map(|r| r.as_str().parse::<u32>().unwrap())
            .collect();

        Claim {
            id: results[0],
            start_row: results[2],
            start_column: results[1],
            width: results[3],
            height: results[4],
        }
    }

    pub fn draw_on(&self, canvas: &mut Canvas) {
        for row in self.start_row..(self.start_row + self.height) {
            for col in self.start_column..(self.start_column + self.width) {
                *canvas.entry((row, col)).or_default() += 1;
            }
        }
    }

    pub fn values_on_canvas(&self, canvas: &Canvas) -> Vec<u32> {
        let mut values = Vec::new();

        for row in self.start_row..(self.start_row + self.height) {
            for col in self.start_column..(self.start_column + self.width) {
                if let Some(value) = canvas.get(&(row, col)) {
                    values.push(*value);
                }
            }
        }

        values
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn claim_new() {
        let input = "#123 @ 3,2: 5x4";
        let claim = Claim::new(input);

        assert_eq!(claim.id, 123);
        assert_eq!(claim.start_row, 2);
        assert_eq!(claim.start_column, 3);
        assert_eq!(claim.width, 5);
        assert_eq!(claim.height, 4);

        let input_2 = "#1353 @ 370,944: 26x15";
        let claim_2 = Claim::new(input_2);

        assert_eq!(claim_2.id, 1353);
        assert_eq!(claim_2.start_row, 944);
        assert_eq!(claim_2.start_column, 370);
        assert_eq!(claim_2.width, 26);
        assert_eq!(claim_2.height, 15);
    }

    #[test]
    fn claim_draw_on_canvas() {
        // ........
        // ...2222.
        // ...2222.
        // .11XX22.
        // .11XX22.
        // .111133.
        // .111133.
        // ........
        let claim_1 = Claim::new("#1 @ 1,3: 4x4");
        let claim_2 = Claim::new("#2 @ 3,1: 4x4");
        let claim_3 = Claim::new("#3 @ 5,5: 2x2");

        let mut expected = Canvas::new();
        expected.insert((1, 3), 1);
        expected.insert((1, 4), 1);
        expected.insert((1, 5), 1);
        expected.insert((1, 6), 1);

        expected.insert((2, 3), 1);
        expected.insert((2, 4), 1);
        expected.insert((2, 5), 1);
        expected.insert((2, 6), 1);

        expected.insert((3, 1), 1);
        expected.insert((3, 2), 1);
        expected.insert((3, 3), 2);
        expected.insert((3, 4), 2);
        expected.insert((3, 5), 1);
        expected.insert((3, 6), 1);

        expected.insert((4, 1), 1);
        expected.insert((4, 2), 1);
        expected.insert((4, 3), 2);
        expected.insert((4, 4), 2);
        expected.insert((4, 5), 1);
        expected.insert((4, 6), 1);

        expected.insert((5, 1), 1);
        expected.insert((5, 2), 1);
        expected.insert((5, 3), 1);
        expected.insert((5, 4), 1);
        expected.insert((5, 5), 1);
        expected.insert((5, 6), 1);

        expected.insert((6, 1), 1);
        expected.insert((6, 2), 1);
        expected.insert((6, 3), 1);
        expected.insert((6, 4), 1);
        expected.insert((6, 5), 1);
        expected.insert((6, 6), 1);

        let mut canvas = Canvas::new();
        claim_1.draw_on(&mut canvas);
        claim_2.draw_on(&mut canvas);
        claim_3.draw_on(&mut canvas);

        assert_eq!(canvas, expected);
    }

    #[test]
    fn claim_get_values() {
        // ........
        // ...2222.
        // ...2222.
        // .11XX22.
        // .11XX22.
        // .111133.
        // .111133.
        // ........
        let claim_1 = Claim::new("#1 @ 1,3: 4x4");
        let claim_2 = Claim::new("#2 @ 3,1: 4x4");
        let claim_3 = Claim::new("#3 @ 5,5: 2x2");

        let mut canvas = Canvas::new();
        claim_1.draw_on(&mut canvas);
        claim_2.draw_on(&mut canvas);
        claim_3.draw_on(&mut canvas);

        let claim_1_values = claim_1.values_on_canvas(&canvas);
        assert_eq!(claim_1_values.iter().filter(|v| **v == 1).count(), 12);
        assert_eq!(claim_1_values.iter().filter(|v| **v == 2).count(), 4);

        let claim_2_values = claim_2.values_on_canvas(&canvas);
        assert_eq!(claim_2_values.iter().filter(|v| **v == 1).count(), 12);
        assert_eq!(claim_2_values.iter().filter(|v| **v == 2).count(), 4);

        let claim_3_values = claim_3.values_on_canvas(&canvas);
        assert_eq!(claim_3_values.iter().filter(|v| **v == 1).count(), 4);
        assert_eq!(claim_3_values.iter().filter(|v| **v == 2).count(), 0);
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

    let mut canvas = Canvas::new();

    let mut claims = Vec::new();
    for line in contents.lines() {
        let claim = Claim::new(line);
        claim.draw_on(&mut canvas);
        claims.push(claim);
    }

    let over_two = canvas.values().filter(|v| **v > 1).count();
    println!("part 1 = {}", over_two);

    for claim in claims {
        let values = claim.values_on_canvas(&canvas);
        if values.iter().filter(|v| **v > 1).count() == 0 {
            println!("part 2 = {}", claim.id);
        }
    }

    Ok(())
}
