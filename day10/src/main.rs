extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn parse_input<'a, I>(lines: I) -> Vec<Position>
where
    I: Iterator<Item = &'a str>,
{
    let line_re: Regex =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)>\svelocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let mut positions = Vec::new();

    for line in lines {
        let caps = line_re.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let velocity_x = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let velocity_y = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        positions.push(Position {
            x,
            y,
            velocity_x,
            velocity_y,
        });
    }

    positions
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Position {
    fn tick(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }
}

struct Grid {
    positions: Vec<Position>,
    height: i32,
    width: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Grid {
    fn new(positions: Vec<Position>) -> Grid {
        let min_x = positions[0].x;
        let max_x = positions[0].x;
        let min_y = positions[0].y;
        let max_y = positions[0].y;

        Grid {
            positions: positions,
            height: 0,
            width: 0,
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }

    fn tick(&mut self) {
        self.min_x = self.positions[0].x;
        self.max_x = self.positions[0].x;
        self.min_y = self.positions[0].y;
        self.max_y = self.positions[0].y;

        for p in self.positions.iter_mut() {
            p.tick();

            if p.x > self.max_x {
                self.max_x = p.x;
            }
            if p.x < self.min_x {
                self.min_x = p.x;
            }
            if p.y > self.max_y {
                self.max_y = p.y;
            }
            if p.y < self.min_y {
                self.min_y = p.y;
            }
        }

        self.height = (self.max_x - self.min_x).abs();
        self.width = (self.max_y - self.min_y).abs();
    }

    fn draw(&self) {
        for x in (0..=self.height).rev() {
            for y in 0..=self.width {
                let mut position_here = false;
                for p in self.positions.iter() {
                    if (p.x - self.min_x) == x && (p.y - self.min_y) == y {
                        position_here = true;
                    }
                }
                if position_here {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input<'a>() -> Vec<&'a str> {
        vec![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>",
        ]
    }

    #[test]
    fn test_parsing_input() {
        let input = test_input();
        let positions = parse_input(input.into_iter());
        assert_eq!(positions.len(), 31);
        assert_eq!(positions[2].x, 3);
        assert_eq!(positions[2].y, -2);
        assert_eq!(positions[2].velocity_x, -1);
        assert_eq!(positions[2].velocity_y, 1);
    }

    #[test]
    fn test_moving_positions() {
        let mut position = Position {
            x: 3,
            y: -2,
            velocity_x: -1,
            velocity_y: 1,
        };

        position.tick();

        assert_eq!(position.x, 2);
        assert_eq!(position.y, -1);
        assert_eq!(position.velocity_x, -1);
        assert_eq!(position.velocity_y, 1);

        position.tick();

        assert_eq!(position.x, 1);
        assert_eq!(position.y, 0);
        assert_eq!(position.velocity_x, -1);
        assert_eq!(position.velocity_y, 1);
    }

    #[test]
    fn test_grid_moving_all_positions() {
        let mut positions = vec![
            Position {
                x: 3,
                y: -2,
                velocity_x: -1,
                velocity_y: 1,
            },
            Position {
                x: 3,
                y: -2,
                velocity_x: -1,
                velocity_y: 1,
            },
        ];

        let mut grid = Grid::new(positions);
        grid.tick();

        assert_eq!(grid.positions[0].x, 2);
        assert_eq!(grid.positions[0].y, -1);
        assert_eq!(grid.positions[1].x, 2);
        assert_eq!(grid.positions[1].y, -1);
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

    let positions = parse_input(contents.lines());
    let mut grid = Grid::new(positions);
    for second in 1..1_000_000 {
        grid.tick();
        if grid.height <= 80 && grid.width <= 20 {
            println!("{} {}s {}", "*".repeat(40), second, "*".repeat(40));
            grid.draw();
        }
    }
    Ok(())
}
