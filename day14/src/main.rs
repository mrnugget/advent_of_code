fn calculate_recipes(input: Vec<i32>, warmup_num: usize, num: usize) -> Vec<i32> {
    let mut recipes = input.clone();
    let mut result = Vec::with_capacity(num);

    let mut index_1 = 0;
    let mut index_2 = 1;

    while result.len() < num {
        // for (i, r) in recipes.iter().enumerate() {
        //     if i == index_1 {
        //         print!("({}) ", r);
        //     } else if i == index_2 {
        //         print!("[{}] ", r);
        //     } else {
        //         print!("{} ", r);
        //     }
        // }
        // print!("\n");

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

        index_1 = ((index_1 as i32 + recipes[index_1] + 1) % recipes.len() as i32) as usize;
        index_2 = ((index_2 as i32 + recipes[index_2] + 1) % recipes.len() as i32) as usize;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculating_the_last_10_recipes() {
        assert_eq!(
            calculate_recipes(vec![3, 7], 9, 10),
            vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]
        );
        assert_eq!(
            calculate_recipes(vec![3, 7], 5, 10),
            vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]
        );
        assert_eq!(
            calculate_recipes(vec![3, 7], 18, 10),
            vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]
        );

        assert_eq!(
            calculate_recipes(vec![3, 7], 2018, 10),
            vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]
        );
    }
}

fn main() {
    // Part 1 - 10 recipes after 702831 recipes
    let recipes = calculate_recipes(vec![3, 7], 702831, 10);
    println!("Part 1 - 10 recipes after 702831 recipes: {:?}", recipes);
    assert_eq!(recipes, vec![1, 1, 3, 2, 4, 1, 3, 1, 1, 1]);
}
