use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

struct Grid {
    coords: Vec<Coord>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(coords: Vec<Coord>) -> Grid {
        let mut width = 0;
        let mut height = 0;

        for c in &coords {
            if c.x > width {
                width = c.x;
            }
            if c.y > height {
                height = c.y;
            }
        }

        Grid {
            coords,
            width,
            height,
        }
    }

    fn find_closest(&self, x: i32, y: i32) -> Option<usize> {
        let mut tied = true;
        let mut closest_idx: usize = 0;
        let mut shortest_distance: i32 = -1;

        for (i, c) in self.coords.iter().enumerate() {
            let dist = c.distance(x, y);

            if dist == shortest_distance && shortest_distance != -1 {
                tied = true;
            }

            if dist < shortest_distance || shortest_distance == -1 {
                tied = false;
                shortest_distance = dist;
                closest_idx = i;
            }
        }

        if tied {
            None
        } else {
            Some(closest_idx)
        }
    }

    fn biggest_area(&self) -> (usize, i32) {
        let mut areas: HashMap<usize, i32> = HashMap::new();
        let mut infinite_areas: HashSet<usize> = HashSet::new();

        for x in 0..=self.width {
            for y in 0..=self.height {
                if let Some(closest) = self.find_closest(x, y) {
                    let count = areas.entry(closest).or_insert(0);
                    *count += 1;
                    if x == 0 || y == 0 || x == self.width || y == self.height {
                        infinite_areas.insert(closest);
                    }
                }
            }
        }

        let mut max_index = 0;
        let mut max_area = 0;

        for (&coord_idx, &coord_area) in areas.iter() {
            if coord_area > max_area && !infinite_areas.contains(&coord_idx) {
                max_index = coord_idx;
                max_area = coord_area;
            }
        }

        (max_index, max_area)
    }

    fn biggest_region(&self) -> i32 {
        let max_dist = 10000;
        let mut region_size = 0;

        for x in 0..=self.width {
            for y in 0..=self.height {
                let mut local_dist = 0;
                for c in self.coords.iter() {
                    local_dist += c.distance(x, y)
                }
                if local_dist < max_dist {
                    region_size += 1;
                }
            }
        }

        region_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_coords() -> Vec<Coord> {
        // ..........
        // .A........
        // ..........
        // ........C.
        // ...D......
        // .....E....
        // .B........
        // ..........
        // ..........
        // ........F.
        vec![
            Coord { x: 1, y: 1 }, // A
            Coord { x: 1, y: 6 }, // B
            Coord { x: 8, y: 3 }, // C
            Coord { x: 3, y: 4 }, // D
            Coord { x: 5, y: 5 }, // E
            Coord { x: 8, y: 9 }, // F
        ]
    }

    #[test]
    fn test_finding_closest() {
        let coords = get_test_coords();
        let grid = Grid::new(coords);
        assert_eq!(grid.find_closest(2, 2), Some(0)); // closest to A
        assert_eq!(grid.find_closest(3, 3), Some(3)); // closest to D
        assert_eq!(grid.find_closest(0, 4), None); // equally close to A and D
        assert_eq!(grid.find_closest(1, 4), None); // equally close to A and D
        assert_eq!(grid.find_closest(0, 5), Some(1)); // closest to B
        assert_eq!(grid.find_closest(6, 0), Some(2)); // closest to C
        assert_eq!(grid.find_closest(5, 2), Some(4)); // closest to E
        assert_eq!(grid.find_closest(6, 8), Some(5)); // closest to F
    }

    #[test]
    fn finding_biggest_area() {
        let coords = get_test_coords();
        let grid = Grid::new(coords);

        let (idx, area_sum) = grid.biggest_area();
        assert_eq!(idx, 4);
        assert_eq!(area_sum, 17);
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

    let coords = contents.lines().fold(Vec::new(), |mut acc, line| {
        let mut splitted = line.split(", ");
        let x = splitted.next().unwrap().parse::<i32>().unwrap();
        let y = splitted.next().unwrap().parse::<i32>().unwrap();
        acc.push(Coord { x, y });
        acc
    });

    // Part 1
    let grid = Grid::new(coords);
    let (coord_idx, area_sum) = grid.biggest_area();

    println!(
        "coordinate {} has the biggest area with {}",
        coord_idx, area_sum,
    );

    // Part 2
    let biggest_region_size = grid.biggest_region();
    println!("the size of the biggest region is {}", biggest_region_size);

    Ok(())
}
