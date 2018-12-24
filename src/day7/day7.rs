use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::collections::{HashSet, BTreeSet};
extern crate petgraph;
use petgraph::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    // let mut map: BTreeMap<char, Vec<char>> = BTreeMap::new();
    // let mut available_steps = BTreeSet::new();
    let mut graph: GraphMap<char, u32, Directed> = GraphMap::new();
    for line in &lines{
        let src = line.chars().nth(5).unwrap();
        let dst = line.chars().nth(36).unwrap();
        // if map.contains_key(&src) == false{
        //     map.insert(src, Vec::new());
        // }

        let mut n = graph.add_node(src);
        graph.add_edge(n, dst, 1);

        // available_steps.insert(src);
        // available_steps.remove(&dst);
        // let mut vec = map.get_mut(&src).unwrap();
        // vec.push(dst);
    }

    let mut open_set = BTreeSet::new();
    let mut num_nodes = 0;
    for n in graph.nodes(){
        num_nodes += 1;
        let incoming = graph.neighbors_directed(n, Direction::Incoming);
        if incoming.count() == 0 {
            open_set.insert(n);
        }
    }


    let mut closed_set: HashSet<char> = HashSet::new();
    let mut order: Vec<char> = Vec::new();
    while false == open_set.is_empty() {
        let mut root = '\0';
        for o in &open_set{
            let mut incoming = graph.neighbors_directed(*o, Direction::Incoming);
            let mut dependencies_satisfied = true;
            incoming.for_each(|n| if false == closed_set.contains(&n){
                dependencies_satisfied = false;
            });
            if dependencies_satisfied{
                root = *o;
                break;
            }
        }

        closed_set.insert(root);
        order.push(root);
        if closed_set.len() == num_nodes {
            break;
        }

        let mut outgoing = graph.neighbors_directed(root, Direction::Outgoing);
        outgoing.for_each(|o| if false == closed_set.contains(&o){
            open_set.insert(o);
        });

        open_set.remove(&root);
        // println!("{:?}", open_set);
        // break;
    }
    let s: String = order.into_iter().collect();
    println!("{:?}", s);
    // for n in order{
        // print!("{:?}", n);
    // }
    // let mut closed_set = BTreeSet::new();

    // println!("{:?}", open_set);


    // let mut bfs = Bfs::new(&graph, a);


    // let mut vec: available_steps = Vec::new();
    // println!("{:?}", available_steps);
    // for (_,v) in map.iter_mut(){
    //     v.sort();
    // }
    // // println!("{:?}", map);

    // for (k, v) in map.iter(){
    //     let a = graph.add_node(*k);
    //     for e in v.iter(){
    //         graph.add_edge(a, *e, 1);
    //     }
    // }
    // println!("{:?}", graph);
}


