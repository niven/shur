use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

use crate::Args;

#[derive(PartialEq, Eq)]
struct Coloring {
    content: Vec<u8>,
}

impl Debug for Coloring {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Coloring({:?})", self.content)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Coloring {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coloring {
    fn cmp(&self, other: &Self) -> Ordering {
        self.content.len().cmp(&other.content.len())
    }
}

pub fn depth_first(args: &Args) -> Option<Vec<u8>> {
    let mut heap = BinaryHeap::new();

    // The initial coloring must have 2 different colors.
    heap.push(Coloring {
        content: [0, 1].to_vec(),
    });

    let mut sentinel = 0;
    while sentinel < args.attempts && heap.len() > 0 {
        let current = heap.pop().unwrap();
        // This looks nice if you uncomment it
        // println!("Current: {:?}", current.content);

        let possible_next_colors = find_next_colors(args.colors, &current);
        for n in possible_next_colors {
            let mut next = current.content.clone();
            next.push(n);
            if next.len() == args.target {
                return Some(next);
            }
            heap.push(Coloring { content: next });
        }

        sentinel += 1;
    }
    if heap.len() == 0 {
        println!("No more candidates in heap");
    }
    return None;
}

// So, so many optimizations possible
fn find_next_colors(colors: u8, c: &Coloring) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut banned: HashSet<u8> = HashSet::new();

    let target = c.content.len();
    // find all sums a+b=c where c is the target. Since a+b=b+a we only need to run a to half target
    for a in 0..=target / 2 {
        let b = target - a - 1;
        if c.content[a] == c.content[b] {
            banned.insert(c.content[a]);
            if banned.len() == colors as usize {
                // If everything is banned there are no possible next colors
                return result;
            }
        }
    }

    for c in 0..colors {
        if !banned.contains(&c) {
            result.push(c);
        }
    }

    return result;
}

// This could also be implemented using a BinaryHeap but that's boring.
pub fn breadth_first(args: &Args) -> Option<Vec<u8>> {
    
    // The initial coloring must have 2 different colors.
    let mut todo: Vec<Vec<u8>> = Vec::new();
    let first = [0, 1];
    todo.push( first.to_vec() );

    let mut sentinel = 0;
    let mut solution_length = 0;
    while sentinel < args.attempts && solution_length < args.target && todo.len() > 0 {
        println!("------ Stack size: {} ------", todo.len() );

        let mut more: Vec<Vec<u8>> = Vec::with_capacity( todo.len() * args.colors as usize );
        for current in todo {
            println!("Current item: {current:?}\n");

            let possible_next_colors = find_next_colors(args.colors, &Coloring{ content: current.clone()});
            println!("{} Children:", possible_next_colors.len());
            for n in possible_next_colors {
                let mut next = current.clone();
                next.push(n);
                println!("\t{:?}", next);
                more.push( next );
            }
        }
        todo = more.to_vec();

        sentinel += 1;
        solution_length += 1;
    }
    if todo.len() == 0 {
        println!("No more candidates in list");
    } else if solution_length < args.target {
        return Some(todo[0].clone());
    }
    
    return None;
}