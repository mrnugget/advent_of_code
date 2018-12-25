fn calculate_recipes(recipes: &mut Vec<i32>, warmup_num: usize, num: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(num);

    let mut index_1 = 0;
    let mut index_2 = 1;

    while result.len() < num {
        let sum = recipes[index_1] + recipes[index_2];
        let digits: Vec<i32> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i32)
            .collect();

        for &d in &digits {
            if recipes.len() >= warmup_num {
                result.push(d);
                if result.len() == num {
                    break;
                }
            }
            recipes.push(d);
        }

        index_1 = (index_1 + recipes[index_1] as usize + 1) % recipes.len();
        index_2 = (index_2 + recipes[index_2] as usize + 1) % recipes.len();
    }

    result
}

fn first_occurrence(recipes: &mut Vec<i32>, pattern: Vec<i32>) -> usize {
    let p_len = pattern.len();

    let mut index_1 = 0;
    let mut index_2 = 1;

    loop {
        let sum = recipes[index_1] + recipes[index_2];
        let digits: Vec<i32> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i32)
            .collect();

        for &d in &digits {
            recipes.push(d);
            let r_len = recipes.len();
            if r_len >= p_len && &recipes[r_len - p_len..] == &pattern[..] {
                return r_len - p_len;
            }
        }

        index_1 = (index_1 + recipes[index_1] as usize + 1) % recipes.len();
        index_2 = (index_2 + recipes[index_2] as usize + 1) % recipes.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_calculating_the_last_10_recipes() {
        assert_eq!(
            calculate_recipes(&mut vec![3, 7], 9, 10),
            vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]
        );
        assert_eq!(
            calculate_recipes(&mut vec![3, 7], 5, 10),
            vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]
        );
        assert_eq!(
            calculate_recipes(&mut vec![3, 7], 18, 10),
            vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]
        );

        assert_eq!(
            calculate_recipes(&mut vec![3, 7], 2018, 10),
            vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]
        );
    }

    #[test]
    fn part_2_find_first_occurence() {
        assert_eq!(first_occurrence(&mut vec![3, 7], vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(first_occurrence(&mut vec![3, 7], vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(first_occurrence(&mut vec![3, 7], vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(first_occurrence(&mut vec![3, 7], vec![5, 9, 4, 1, 4]), 2018);
    }
}

fn main() {
    // Part 1 - 10 recipes after 702831 recipes
    let mut input = vec![3, 7];
    let recipes = calculate_recipes(&mut input, 702831, 10);
    println!("Part 1 - 10 recipes after 702831 recipes: {:?}", recipes);
    assert_eq!(recipes, vec![1, 1, 3, 2, 4, 1, 3, 1, 1, 1]);

    // Part 2 - calculate after how many receipes the given pattern shows up
    let first_occurrence = first_occurrence(&mut vec![3, 7], vec![7, 0, 2, 8, 3, 1]);
    println!(
        "Part 2 - {} to the left of the pattern 702831",
        first_occurrence
    );
    assert_eq!(first_occurrence, 20340232);
}
