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

        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let start_column = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let start_row = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let width = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let height = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

        Claim {
            id: id,
            start_row: start_row,
            start_column: start_column,
            width: width,
            height: height,
        }
    }

    pub fn draw_on(&self, canvas: &mut Canvas) {
        for row in self.start_row..(self.start_row + self.height) {
            for col in self.start_column..(self.start_column + self.width) {
                let cell = canvas.entry((row, col)).or_insert(0);
                *cell += 1;
            }
        }
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
        println!("claim_1 = {:?}", claim_1);
        let claim_2 = Claim::new("#2 @ 3,1: 4x4");
        println!("claim_2 = {:?}", claim_2);
        let claim_3 = Claim::new("#3 @ 5,5: 2x2");
        println!("claim_3 = {:?}", claim_1);

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
        println!("claim_1 drawing");
        claim_1.draw_on(&mut canvas);
        println!("claim_2 drawing");
        claim_2.draw_on(&mut canvas);
        println!("claim_3 drawing");
        claim_3.draw_on(&mut canvas);

        assert_eq!(canvas, expected);
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

    for line in contents.lines() {
        let claim = Claim::new(line);
        claim.draw_on(&mut canvas);
    }

    println!("canvas={:?}", canvas);

    let over_two = canvas.values().filter(|v| **v > 1).count();
    println!("over_two={}", over_two);
    Ok(())
}
