use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

use crate::Args;
use crate::util;
use rand;

pub fn random_fill(colors: u8, n: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(n);
    for _ in 0..n {
        let color: u8 = rand::random_range(0..colors);
        vec.push(color);
    }
    return vec;
}

// Just assign colors at random
pub fn random_assignment(args: &Args) -> Option<Vec<u8>> {
    for _ in 0..args.attempts {
        let result = random_fill(args.colors, args.target);
        if let Ok(_) = util::check_coloring(&result) {
            return Some(result);
        }
    }

    return None;
}

// Assign colors at random, but whenever assigning add
// numbers to a "ban list"
// After adding 3 as color C, ban color C for 3+3 as well as any
// 3 + n<3 with color C
pub fn random_with_bannings(args: &Args) -> Option<Vec<u8>> {
    let colors = args.colors;
    let target = args.target;

    for _ in 0..args.attempts {
        let mut bans: HashMap<usize, HashSet<u8>> = HashMap::new();

        let mut vec = Vec::with_capacity(target);
        for current in 0..target {
            let mut col: u8 = rand::random_range(0..colors);
            // println!("Initial col: {col}");
            // Check bans
            if let Some(banned_colors) = bans.get(&current)
                && banned_colors.contains(&col)
            {
                // println!("Banned: {banned_colors:?}");

                // Just pick the first one that is available
                for c in 0..colors {
                    if !banned_colors.contains(&c) {
                        col = c;
                        break;
                    }
                }
            }
            // println!("Final col: {col}");
            vec.push(col);
            // One could now delete the bans for this number, but that's just work and the memory overhead is minimal

            // Add bans up to the target. If target = 5 no need for bans for 4+4
            let mut n = 0;
            while n <= current && current + n + 1 < target {
                // println!("Banning {current}({}) + {n}({}) if col = match", vec[current], vec[n]);
                if vec[n] == col {
                    let ban_key = current + n + 1; // +2 for the current and n being 0-indexed, -1 for the result 
                    let ban_val = bans.entry(ban_key).or_insert(HashSet::new());
                    ban_val.insert(col);
                }
                n += 1;
            }
        }

        if let Ok(_) = util::check_coloring(&vec) {
            return Some(vec);
        }
    }

    return None;
}

// Assign colors at random, keep any prefixes (prefix: sequence of color assignments starting at 1 up to N) that are avlid.
// Use a priority queue (binary heap) to keep every prefix with the count of times it has been used.
// Then, select the longest prefix with the lowest use count and extend it randomly. Insert with count = 1.
// This means we do essentially an optimistic and random Depth First Search but we back off after trying unsuccesful prefixes.
// Example:
/*
* 1, ""
* 2, "abba"
* 1, "accbe"
*
* At this point select "accbe" to extend and set it's count to 2. If that fails, pick "" and extend it.
*/
#[derive(PartialEq, Eq)]
struct Prefix {
    count: usize,
    content: Vec<u8>,
}

impl Debug for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Prefix(count: {}, len: {})",
            self.count,
            self.content.len()
        )
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Prefix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Prefix {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on count to get the lowest first
        // Second ordering is on length of the prefix: longest first
        other
            .count
            .cmp(&self.count)
            .then_with(|| self.content.len().cmp(&other.content.len()))
    }
}

pub fn random_with_backtrack(args: &Args) -> Option<Vec<u8>> {
    let (colors, target) = (args.colors, args.target);

    let mut heap = BinaryHeap::new();
    // Initial prefix with 0 assignments
    // Since 1 and 2 are by definition different, this saves generating the
    // same prefixes.
    heap.push(Prefix {
        count: 0,
        content: [0, 1].to_vec(),
    });

    let mut sentinel = 0;
    while sentinel < args.attempts {
        // println!("Heap: {heap:?}");
        let mut prefix = heap.pop().unwrap();
        // println!("Current: {}, {:?}", prefix.count, prefix.content);
        prefix.count += 1;

        let mut candidate = prefix.content.clone();
        let required_len = target - prefix.content.len();
        let remainder = random_fill(colors, required_len);
        candidate.extend(remainder);
        // println!("Candidate: {:?}", candidate);

        match util::check_coloring(&candidate) {
            Ok(_) => {
                return Some(candidate);
            }
            Err(err) => {
                // println!("Failed at index {}: {}", err.index, err.message);
                // Remove everything from the failed index to the end.
                candidate = candidate[0..err.index].to_vec();
                // If the candidate is the same as current, avoid inserting a copy
                if candidate.len() > prefix.content.len() {
                    heap.push(Prefix {
                        count: 0,
                        content: candidate,
                    });
                }
                heap.push(prefix); // always put back the one we used                
            }
        }
        sentinel += 1;
    }

    return None;
}
