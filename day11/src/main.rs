struct Grid {
    serial_number: i32,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(serial_number: i32) -> Grid {
        Grid {
            serial_number,
            width: 300,
            height: 300,
        }
    }

    fn power_level_at(&self, x: i32, y: i32) -> i32 {
        let rack_id = x + 10;
        let mut power_level = ((rack_id * y) + self.serial_number) * rack_id;
        power_level = (power_level / 100) % 10;
        power_level - 5
    }

    fn square_power_level_at(&self, top_left_x: i32, top_left_y: i32) -> i32 {
        let mut sum = 0;
        for x in top_left_x..top_left_x + 3 {
            if x > self.width {
                break;
            }

            for y in top_left_y..top_left_y + 3 {
                if y > self.width {
                    break;
                }

                let power_level = self.power_level_at(x, y);
                sum += power_level;
            }
        }
        sum
    }

    fn highest_powered_square(&self) -> (i32, i32) {
        let mut max_power = 0;
        let mut max_coords = (0, 0);

        for x in 1..=self.width {
            for y in 1..=self.height {
                let square_power = self.square_power_level_at(x, y);
                if square_power > max_power {
                    max_power = square_power;
                    max_coords = (x, y);
                }
            }
        }

        max_coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_levels() {
        assert_eq!(Grid::new(8).power_level_at(3, 5), 4);
        assert_eq!(Grid::new(57).power_level_at(122, 79), -5);
        assert_eq!(Grid::new(39).power_level_at(217, 196), 0);
        assert_eq!(Grid::new(71).power_level_at(101, 153), 4);
    }

    #[test]
    fn square_power_levels() {
        assert_eq!(Grid::new(18).square_power_level_at(33, 45), 29);
        assert_eq!(Grid::new(42).square_power_level_at(21, 61), 30);
    }

    #[test]
    fn highest_powered_square() {
        assert_eq!(Grid::new(18).highest_powered_square(), (33, 45));
        assert_eq!(Grid::new(42).highest_powered_square(), (21, 61));
    }
}

fn main() {
    let grid_serial_number = 5177;
    let grid = Grid::new(grid_serial_number);

    // Part 1
    let (x, y) = grid.highest_powered_square();
    println!("highest powered square at: {},{}", x, y);
    assert_eq!(x, 235);
    assert_eq!(y, 22);
}
