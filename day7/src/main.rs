extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    parent: Option<&'a Node<'a>>,
    children: Vec<&'a Node<'a>>,
}

fn find_or_insert<'a>(nodes: &'a mut Vec<Node<'a>>, name: &'a str) -> &'a Node<'a> {
    match nodes.iter().find(|n| n.name == name) {
        Some(n) => n,
        None => {
            let n = Node {
                name: name,
                parent: None,
                children: Vec::new(),
            };
            nodes.push(n);
            nodes.last().unwrap()
        }
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

    let line_re: Regex = Regex::new(r"Step\s(\w)\smust\sbe\sfinished\sbefore\sstep\s(\w)").unwrap();

    let _relationships = contents.lines().fold(Vec::new(), |mut acc, line| {
        let caps = line_re.captures(line).unwrap();
        let parent = caps.get(1).unwrap().as_str();
        let child = caps.get(2).unwrap().as_str();
        acc.push((parent, child));
        acc
    });

    // let mut nodes: Vec<Node> = Vec::new();
    //
    // for (p, c) in relationships.iter() {
    //     match nodes.iter().find(|n| n.name == *p) {
    //         None => {
    //             let child = match nodes.iter().find(|n| n.name == *c) {
    //                 None => {
    //                     nodes.push(Node {
    //                         name: *c,
    //                         parent: None,
    //                         children: Vec::new(),
    //                     });
    //                     nodes.last().unwrap()
    //                 }
    //                 Some(c) => c,
    //             };
    //
    //             nodes.push(Node {
    //                 name: *p,
    //                 parent: None,
    //                 children: vec![child],
    //             });
    //         }
    //         Some(_) => {}
    //     };
    //
    //     ;
    // }
    //
    // println!("nodes={:?}", nodes);

    Ok(())
}
