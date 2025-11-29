use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use crate::Args;
use crate::util::ColorVec;

pub fn depth_first(args: &Args) -> Option<ColorVec> {
    let mut heap: BinaryHeap<ColorVec> = BinaryHeap::new();

    // The initial coloring must have 2 different colors.
    heap.push( [0, 1].to_vec() );

    let mut sentinel = 0;
    while sentinel < args.attempts && heap.len() > 0 {
        let current = heap.pop().unwrap();
        // This looks nice if you uncomment it
        // println!("Current: {:?}", current.content);

        let possible_next_colors = find_next_colors(args.colors, &current);
        for n in possible_next_colors {
            let mut next = current.clone();
            next.push(n);
            if next.len() == args.target {
                return Some(next);
            }
            heap.push( next );
        }

        sentinel += 1;
    }
    if heap.len() == 0 {
        println!("No more candidates in heap");
    }
    return None;
}

// So, so many optimizations possible
fn find_next_colors(colors: u8, c: &ColorVec) -> Vec<u8> {
    let mut result: ColorVec = Vec::new();
    let mut banned: HashSet<u8> = HashSet::new();

    let target = c.len();
    // find all sums a+b=c where c is the target. Since a+b=b+a we only need to run a to half target
    for a in 0..=target / 2 {
        let b = target - a - 1;
        if c[a] == c[b] {
            banned.insert(c[a]);
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
pub fn breadth_first(args: &Args) -> Option<ColorVec> {
    
    // The initial coloring must have 2 different colors.
    let mut todo: Vec<ColorVec> = Vec::new();
    let first = [0, 1];
    todo.push( first.to_vec() );

    let mut sentinel = 0;
    let mut solution_length = 0;
    while sentinel < args.attempts && solution_length < args.target && todo.len() > 0 {
        println!("------ Stack size: {} ------", todo.len() );

        let mut more: Vec<ColorVec> = Vec::with_capacity( todo.len() * args.colors as usize );
        for current in todo {
            println!("Current item: {current:?}\n");

            let possible_next_colors = find_next_colors(args.colors, &current);
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