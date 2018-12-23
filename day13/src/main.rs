use std::env;
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
struct Cart {
    direction: Direction,
}

#[derive(PartialEq, Debug, Clone)]
enum TrackElement {
    Horizontal,
    Vertical,
    TopLeftToBottomRight,
    TopRightToLeftBottom,
    Intersection,
}

struct Grid {
    width: usize,
    height: usize,
    tracks: Vec<Vec<Option<TrackElement>>>,
    carts: Vec<Vec<Option<Cart>>>,
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
                    '>' | '<' | 'v' | '^' => {
                        if tracks[x][y - 1] == Some(TrackElement::Horizontal) {
                            Some(TrackElement::Horizontal)
                        } else if tracks[x - 1][y] == Some(TrackElement::Vertical)
                            || tracks[x - 1][y] == Some(TrackElement::TopRightToLeftBottom)
                            || tracks[x - 1][y] == Some(TrackElement::TopLeftToBottomRight)
                        {
                            Some(TrackElement::Vertical)
                        } else {
                            None
                        }
                    }
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
                    carts[x][y].replace(Cart { direction });
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
        }
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

        assert_eq!(
            grid.carts[0][2],
            Some(Cart {
                direction: Direction::Right
            })
        );
        assert_eq!(
            grid.carts[3][9],
            Some(Cart {
                direction: Direction::Down
            })
        );
    }

    #[test]
    fn interpolating_track_elements_underneath_carts() {
        let grid = Grid::from_string(String::from(INPUT));

        assert_eq!(grid.tracks[0][2], Some(TrackElement::Horizontal));
        println!("{:?}", grid.tracks[2][9]);
        assert_eq!(grid.tracks[3][9], Some(TrackElement::Vertical));
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

    let grid = Grid::from_string(contents);
    println!("grid.height={}, grid.width={}", grid.height, grid.width);
    println!("grid.carts={:?}, grid.tracks={:?}", grid.carts, grid.tracks);

    Ok(())
}
