pub const COLOR_LETTERS: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

pub type Solution = Vec<u8>;

#[derive(Debug)]
pub struct InvalidOrdering {
    pub index: usize,
    pub message: String,
}

pub fn check_coloring(colors: &Vec<u8>) -> Result<&str, InvalidOrdering> {
    let max = colors.len() as u64;

    // Use inclusive ranges
    for a in 2..=max as usize {
        for b in 1..=a / 2 as usize {
            // println!("\t{} + {} = {} => {} + {} = {}", b,  a-b, a, colors[b-1], colors[a-b-1], colors[a-1]);
            if colors[b - 1] == colors[a - b - 1] && colors[a - b - 1] == colors[a - 1] {
                let invalid = InvalidOrdering {
                    index: a - 1,
                    message: format!(
                        "Invalid coloring: {}({}) + {}({}) = {}({})",
                        b,
                        colors[b - 1],
                        a - b,
                        colors[a - b - 1],
                        a,
                        colors[a - 1]
                    ),
                };
                return Err(invalid);
            }
        }
    }

    return Ok("Coloring is valid");
}

/**
These are basically the checks that need to happen:
1	-
2	1+1
3	1+2
4	1+3	2+2
5	1+4	2+3
6	1+5	2+4	3+3
7	1+6	2+5	3+4
8	1+7	2+6	3+5	4+4

So for checking N numbers you need:
    if N is odd:  2*T( (n-1)/2 )
    if N is even: checks(n-1) + n/2
Where T(n) is the triangular number for n
 */
fn num_checks(n: u64) -> u64 {
    if n % 2 == 0 {
        return num_checks(n - 1) + n / 2;
    }

    let t_n = triangle((n - 1) / 2);
    return 2 * t_n;
}

fn triangle(n: u64) -> u64 {
    return n * (n + 1) / 2;
}

// Format a vector of numbers into a human readable string
pub fn short(solution: &Vec<u8>) -> String {
    let mapped: Vec<char> = solution
        .iter()
        .map(|&i| COLOR_LETTERS[i as usize])
        .collect();
    return mapped
        .into_iter()
        .fold(String::from(""), |a, b| format!("{a}{b}"));
}
