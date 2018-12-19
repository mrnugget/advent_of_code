// use std::collections::HashMap;
// use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(" ")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

fn build_graph<'a, I>(input: &mut I) -> Node
where
    I: Iterator<Item = &'a u32>,
{
    let num_children = input.next().unwrap();
    let num_metadata = input.next().unwrap();

    let mut children: Vec<Node> = Vec::new();
    for _ in 0..*num_children {
        children.push(build_graph(input));
    }

    let mut metadata: Vec<u32> = Vec::new();
    for _ in 0..*num_metadata {
        metadata.push(*input.next().unwrap());
    }

    Node { children, metadata }
}

fn sum_metadata(n: &Node) -> u32 {
    let mut sum = n.metadata.iter().sum();

    for c in n.children.iter() {
        sum += sum_metadata(c);
    }

    sum
}

fn value_of_node(n: &Node) -> u32 {
    if n.children.is_empty() {
        return n.metadata.iter().sum();
    }

    let mut value: u32 = 0;

    for &meta in n.metadata.iter() {
        let i = meta - 1;
        if let Some(child) = n.children.get(i as usize) {
            value += value_of_node(child);
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input<'a>() -> &'a str {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
    }

    #[test]
    fn test_parsing_input() {
        let input = test_input();
        assert_eq!(
            parse_input(input),
            vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
        );
    }

    #[test]
    fn test_building_graph() {
        let input = parse_input(test_input());

        let mut iter = input.iter();
        let root = build_graph(&mut iter);
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.metadata, vec![1, 1, 2]);

        assert_eq!(root.children[0].children.len(), 0);
        assert_eq!(root.children[0].metadata, vec![10, 11, 12]);

        assert_eq!(root.children[1].children.len(), 1);
        assert_eq!(root.children[1].metadata, vec![2]);

        assert_eq!(root.children[1].children[0].children.len(), 0);
        assert_eq!(root.children[1].children[0].metadata, vec![99]);
    }

    #[test]
    fn test_summing_up_metadata() {
        let input = parse_input(test_input());
        let mut iter = input.iter();
        let root = build_graph(&mut iter);

        let sum = sum_metadata(&root);
        assert_eq!(sum, 138);
    }

    #[test]
    fn calculating_value_of_node() {
        let input = parse_input(test_input());
        let mut iter = input.iter();
        let root = build_graph(&mut iter);

        let value = value_of_node(&root);
        assert_eq!(value, 66);
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

    let first_line = contents.lines().next().unwrap();
    let input = parse_input(first_line);
    let mut iter = input.iter();
    let root = build_graph(&mut iter);

    // Part 1
    let metadata_sum = sum_metadata(&root);
    println!("metadata_sum={}", metadata_sum);
    // Correct result
    assert_eq!(metadata_sum, 42798);

    // Part 2
    let value_root_node = value_of_node(&root);
    println!("value_root_node={}", value_root_node);
    assert_eq!(value_root_node, 23798);

    Ok(())
}
