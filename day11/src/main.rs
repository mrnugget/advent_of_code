struct Grid {
    serial_number: i32,
    width: i32,
    height: i32,
    powers: Vec<Vec<i32>>,
}

impl Grid {
    fn new(serial_number: i32) -> Grid {
        let powers = vec![vec![0; 300]; 300];

        let mut grid = Grid {
            serial_number,
            width: 300,
            height: 300,
            powers: powers,
        };
        grid.precalculate_power_levels();
        grid
    }

    fn precalculate_power_levels(&mut self) {
        for x in 1..=self.width {
            for y in 1..=self.height {
                self.powers[(x - 1) as usize][(y - 1) as usize] = self.power_level_at(x, y);
            }
        }
    }

    fn get_power_level_at(&self, x: i32, y: i32) -> i32 {
        self.powers[(x - 1) as usize][(y - 1) as usize]
    }

    fn power_level_at(&self, x: i32, y: i32) -> i32 {
        let rack_id = x + 10;
        let mut power_level = ((rack_id * y) + self.serial_number) * rack_id;
        power_level = (power_level / 100) % 10;
        power_level - 5
    }

    fn square_power_level_at(&self, square_size: i32, top_left_x: i32, top_left_y: i32) -> i32 {
        let mut sum = 0;
        for x in top_left_x..top_left_x + square_size {
            if x > self.width {
                break;
            }

            for y in top_left_y..top_left_y + square_size {
                if y > self.width {
                    break;
                }

                sum += self.get_power_level_at(x, y);
            }
        }
        sum
    }

    fn highest_powered_square_with_size(&self, square_size: i32) -> (i32, i32) {
        let mut max_power = 0;
        let mut max_coords = (0, 0);

        for x in 1..=self.width {
            for y in 1..=self.height {
                let square_power = self.square_power_level_at(square_size, x, y);
                if square_power > max_power {
                    max_power = square_power;
                    max_coords = (x, y);
                }
            }
        }

        max_coords
    }

    fn find_highest_powered_square(&self) -> (i32, i32, i32) {
        let mut max_power = 0;
        let mut max_coords_and_size = (0, 0, 0);

        for x in 1..=self.width {
            for y in 1..=self.height {
                println!("x={}, y={}", x, y);
                for square_size in 0..(self.width - x + 1) {
                    let square_power = self.square_power_level_at(square_size, x, y);
                    if square_power > max_power {
                        max_power = square_power;
                        max_coords_and_size = (x, y, square_size);
                    }
                }
            }
        }

        max_coords_and_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_levels() {
        assert_eq!(Grid::new(8).get_power_level_at(3, 5), 4);
        assert_eq!(Grid::new(57).get_power_level_at(122, 79), -5);
        assert_eq!(Grid::new(39).get_power_level_at(217, 196), 0);
        assert_eq!(Grid::new(71).get_power_level_at(101, 153), 4);
    }

    #[test]
    fn square_power_levels() {
        assert_eq!(Grid::new(18).square_power_level_at(3, 33, 45), 29);
        assert_eq!(Grid::new(18).square_power_level_at(16, 90, 269), 113);
        assert_eq!(Grid::new(42).square_power_level_at(3, 21, 61), 30);
        assert_eq!(Grid::new(42).square_power_level_at(12, 232, 251), 119);
    }

    #[test]
    fn highest_powered_square_with_size() {
        assert_eq!(Grid::new(18).highest_powered_square_with_size(3), (33, 45));
        assert_eq!(Grid::new(42).highest_powered_square_with_size(3), (21, 61));
    }

    #[test]
    fn find_highest_powered_square() {
        // assert_eq!(Grid::new(18).find_highest_powered_square(), (90, 269, 16));
        // assert_eq!(Grid::new(42).find_highest_powered_square(), (232, 251, 12));
    }
}

fn main() {
    let grid_serial_number = 5177;
    let grid = Grid::new(grid_serial_number);

    // Part 1
    let (x, y) = grid.highest_powered_square_with_size(3);
    println!("highest powered square with size 3 is at: {},{}", x, y);
    assert_eq!(x, 235);
    assert_eq!(y, 22);

    // Part 2
    let (x, y, size) = grid.find_highest_powered_square();
    println!(
        "highest powered square with size {} is at: {},{}",
        size, x, y
    );
    assert_eq!(x, 231);
    assert_eq!(y, 135);
    assert_eq!(size, 8);
}
