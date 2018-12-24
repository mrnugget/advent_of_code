use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Debug, Clone)]
enum IntersectionDirection {
    Left,
    Straight,
    Right,
}

impl fmt::Display for IntersectionDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::IntersectionDirection::*;

        let output = match self {
            Left => "LEFT",
            Straight => "STRAIGHT",
            Right => "RIGHT",
        };
        return write!(f, "{}", output);
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Cart {
    direction: Direction,
    intersection_direction: IntersectionDirection,
}

impl Cart {
    fn new(direction: Direction) -> Cart {
        Cart {
            direction,
            intersection_direction: IntersectionDirection::Left,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum TrackElement {
    Horizontal,
    Vertical,
    TopLeftToBottomRight,
    TopRightToLeftBottom,
    Intersection,
}

type Tracks = Vec<Vec<Option<TrackElement>>>;
type Carts = Vec<Vec<Option<Cart>>>;

struct Grid {
    width: usize,
    height: usize,
    tracks: Tracks,
    carts: Carts,
    num_carts: u32,
}

fn interpolate_track_element(tracks: &Tracks, x: usize, y: usize) -> Option<TrackElement> {
    if tracks[x][y - 1] == Some(TrackElement::Horizontal) {
        return Some(TrackElement::Horizontal);
    }

    if tracks[x][y - 1] == Some(TrackElement::Intersection) {
        return Some(TrackElement::Horizontal);
    }

    if tracks[x][y - 1] == Some(TrackElement::TopLeftToBottomRight) {
        return Some(TrackElement::Horizontal);
    }
    if tracks[x][y - 1] == Some(TrackElement::TopRightToLeftBottom) {
        return Some(TrackElement::Horizontal);
    }

    if let Some(up) = &tracks[x - 1][y] {
        return match up {
            TrackElement::Vertical
            | TrackElement::TopRightToLeftBottom
            | TrackElement::TopLeftToBottomRight
            | TrackElement::Intersection => Some(TrackElement::Vertical),
            _ => None,
        };
    }

    return None;
}

impl Grid {
    fn from_string(grid_string: String) -> Grid {
        let mut height = 0;
        let mut width = 0;

        for line in grid_string.lines() {
            let line_width = line.chars().count();
            if line_width > width {
                width = line_width;
            }

            height += 1;
        }

        let mut tracks = vec![vec![None; width]; height];
        let mut carts = vec![vec![None; width]; height];

        let mut num_carts = 0;
        let mut x = 0;
        let mut y = 0;

        for line in grid_string.lines() {
            for c in line.chars() {
                let track_element = match c {
                    '|' => Some(TrackElement::Vertical),
                    '-' => Some(TrackElement::Horizontal),
                    '+' => Some(TrackElement::Intersection),
                    '\\' => Some(TrackElement::TopLeftToBottomRight),
                    '/' => Some(TrackElement::TopRightToLeftBottom),
                    '>' | '<' | 'v' | '^' => interpolate_track_element(&tracks, x, y),
                    _ => None,
                };
                if let Some(element) = track_element {
                    tracks[x][y].replace(element);
                }

                let cart_direction = match c {
                    '>' => Some(Direction::Right),
                    '<' => Some(Direction::Left),
                    'v' => Some(Direction::Down),
                    '^' => Some(Direction::Up),
                    _ => None,
                };
                if let Some(direction) = cart_direction {
                    carts[x][y].replace(Cart::new(direction));
                    num_carts += 1;
                }

                y += 1;
            }

            y = 0;
            x += 1;
        }

        Grid {
            width,
            height,
            tracks,
            carts,
            num_carts,
        }
    }

    fn move_carts(&mut self, stop_on_crash: bool) -> Result<(), (usize, usize)> {
        use self::Direction::*;
        use self::TrackElement::*;

        let mut new_carts = self.carts.clone();
        for x in 0..self.height {
            for y in 0..self.width {
                if let Some(cart) = &mut self.carts[x][y] {
                    let direction = cart.direction.clone();

                    let (new_x, new_y) = match direction {
                        Up => (x - 1, y),
                        Down => (x + 1, y),
                        Left => (x, y - 1),
                        Right => (x, y + 1),
                    };

                    if new_carts[new_x][new_y].is_some() {
                        new_carts[x][y] = None;
                        new_carts[new_x][new_y] = None;
                        self.carts[new_x][new_y] = None;
                        self.num_carts -= 2;
                        if stop_on_crash {
                            return Err((new_y, new_x));
                        } else {
                            continue;
                        }
                    }

                    let intersection_direction = cart.intersection_direction.clone();

                    let new_direction = match self.tracks[new_x][new_y] {
                        Some(Horizontal) => direction,
                        Some(Vertical) => direction,
                        Some(TopRightToLeftBottom) => match direction {
                            Up => Right,
                            Down => Left,
                            Right => Up,
                            Left => Down,
                        },
                        Some(TopLeftToBottomRight) => match direction {
                            Up => Left,
                            Down => Right,
                            Left => Up,
                            Right => Down,
                        },
                        Some(Intersection) => match intersection_direction {
                            IntersectionDirection::Straight => direction,
                            IntersectionDirection::Left => match direction {
                                Up => Left,
                                Down => Right,
                                Left => Down,
                                Right => Up,
                            },
                            IntersectionDirection::Right => match direction {
                                Up => Right,
                                Down => Left,
                                Left => Up,
                                Right => Down,
                            },
                        },
                        None => panic!(
                            "Cart runs off track at {},{} (origin: {}, {})",
                            new_x, new_y, x, y
                        ),
                    };

                    let new_intersection_direction = match self.tracks[new_x][new_y] {
                        Some(Horizontal) => intersection_direction,
                        Some(Vertical) => intersection_direction,
                        Some(TopRightToLeftBottom) => intersection_direction,
                        Some(TopLeftToBottomRight) => intersection_direction,
                        Some(Intersection) => match intersection_direction {
                            IntersectionDirection::Left => IntersectionDirection::Straight,
                            IntersectionDirection::Straight => IntersectionDirection::Right,
                            IntersectionDirection::Right => IntersectionDirection::Left,
                        },
                        None => panic!("Cart ran off track"),
                    };

                    new_carts[x][y] = None;
                    new_carts[new_x][new_y].replace(Cart {
                        direction: new_direction,
                        intersection_direction: new_intersection_direction,
                    });
                }
            }
        }

        self.carts = new_carts;
        Ok(())
    }

    fn cart_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        for x in 0..self.height {
            for y in 0..self.width {
                if let Some(_) = &self.carts[x][y] {
                    positions.push((x, y));
                }
            }
        }

        positions
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 15..35 {
            write!(f, "[{}]\t", x)?;
            for y in 0..self.width {
                if let Some(cart) = &self.carts[x][y] {
                    let car_format = match cart.direction {
                        Direction::Up => "^",
                        Direction::Down => "v",
                        Direction::Left => "<",
                        Direction::Right => ">",
                    };
                    write!(f, "{}", car_format)?;
                } else if let Some(track) = &self.tracks[x][y] {
                    let track_format = match track {
                        TrackElement::Horizontal => "-",
                        TrackElement::Vertical => "|",
                        TrackElement::TopRightToLeftBottom => "/",
                        TrackElement::TopLeftToBottomRight => "\\",
                        TrackElement::Intersection => "+",
                    };
                    write!(f, "{}", track_format)?;
                } else {
                    write!(f, "{}", " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   

"#;

    #[test]
    fn determine_width_height() {
        let grid = Grid::from_string(String::from(INPUT));
        assert_eq!(grid.width, 13);
        assert_eq!(grid.height, 7);
    }

    #[test]
    fn parsing_track_elements() {
        let grid = Grid::from_string(String::from(INPUT));

        assert_eq!(grid.tracks[0][0], Some(TrackElement::TopRightToLeftBottom));
        assert_eq!(grid.tracks[0][1], Some(TrackElement::Horizontal));
        assert_eq!(grid.tracks[0][3], Some(TrackElement::Horizontal));
        assert_eq!(grid.tracks[0][4], Some(TrackElement::TopLeftToBottomRight));

        assert_eq!(
            grid.tracks[1][0..=5],
            vec![
                Some(TrackElement::Vertical),
                None,
                None,
                None,
                Some(TrackElement::Vertical),
                None
            ][..]
        );

        assert_eq!(
            grid.tracks[2][0..=7],
            vec![
                Some(TrackElement::Vertical),
                None,
                Some(TrackElement::TopRightToLeftBottom),
                Some(TrackElement::Horizontal),
                Some(TrackElement::Intersection),
                Some(TrackElement::Horizontal),
                Some(TrackElement::Horizontal),
                Some(TrackElement::Intersection),
            ][..]
        );
    }

    #[test]
    fn parsing_cart_positions() {
        let grid = Grid::from_string(String::from(INPUT));

        assert_eq!(grid.carts[0][2], Some(Cart::new(Direction::Right)));
        assert!(grid.carts[1][..].iter().all(|c| c.is_none()));
        assert!(grid.carts[2][..].iter().all(|c| c.is_none()));
        assert_eq!(grid.carts[3][9], Some(Cart::new(Direction::Down)));
        assert!(grid.carts[4][..].iter().all(|c| c.is_none()));
        assert!(grid.carts[5][..].iter().all(|c| c.is_none()));
        assert!(grid.carts[6][..].iter().all(|c| c.is_none()));
    }

    #[test]
    fn interpolating_track_elements_underneath_carts() {
        let grid = Grid::from_string(String::from(INPUT));

        assert_eq!(grid.tracks[0][2], Some(TrackElement::Horizontal));
        assert_eq!(grid.tracks[3][9], Some(TrackElement::Vertical));
    }

    #[test]
    fn moving_carts_on_grid() {
        let mut grid = Grid::from_string(String::from(INPUT));

        //    /->-\
        //    |   |  /----\
        //    | /-+--+-\  |
        //    | | |  | v  |
        //    \-+-/  \-+--/
        //      \------/

        assert_eq!(grid.carts[0][2], Some(Cart::new(Direction::Right)));
        assert_eq!(grid.carts[3][9], Some(Cart::new(Direction::Down)));

        assert!(grid.move_carts(true).is_ok());

        //    /-->\
        //    |   |  /----\
        //    | /-+--+-\  |
        //    | | |  | |  |
        //    \-+-/  \->--/
        //      \------/
        assert_eq!(grid.carts[0][3], Some(Cart::new(Direction::Right)));
        assert_eq!(
            grid.carts[4][9].clone().unwrap().direction,
            Direction::Right
        );
        assert_eq!(
            grid.carts[4][9].clone().unwrap().intersection_direction,
            IntersectionDirection::Straight
        );

        for _ in 0..12 {
            assert!(grid.move_carts(true).is_ok());
        }

        assert_eq!(grid.move_carts(true), Err((7, 3)));
    }

    #[test]
    fn moving_carts_until_one_is_left() {
        let crash_input = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

        let mut grid = Grid::from_string(String::from(crash_input));

        loop {
            let result = grid.move_carts(false);
            assert!(result.is_ok());
            if grid.num_carts == 1 {
                break;
            }
        }

        let cart_positions = grid.cart_positions();
        assert_eq!(cart_positions.len(), 1);
        assert_eq!(cart_positions.first(), Some(&(4, 6)));
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

    // Part 1
    let mut part_1_grid = Grid::from_string(contents.clone());
    loop {
        if let Err((x, y)) = part_1_grid.move_carts(true) {
            println!("Part 1 - first crash at {},{}", x, y);
            assert_eq!((x, y), (32, 99));
            break;
        }
    }

    // Part 2
    let mut part_2_grid = Grid::from_string(contents.clone());
    loop {
        if let Err((x, y)) = part_2_grid.move_carts(false) {
            println!("Part 2 - unexpected crash at {},{}", x, y);
            break;
        }
        if part_2_grid.num_carts == 1 {
            break;
        }
    }
    assert_eq!(part_2_grid.cart_positions().len(), 1);
    if let Some((y, x)) = part_2_grid.cart_positions().first() {
        println!("Part 2 - last cart at {},{}", x, y);
    }

    Ok(())
}
