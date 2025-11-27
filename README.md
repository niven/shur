# Schur Numbers

## Numberphile episode
https://www.youtube.com/watch?v=57V8Ud7PL8k

# Definition

A coloring of the numbers `{1, 2, ..., n}` is valid if:
- No two numbers of the same color can sum to another number of the same color

## Known numbers
S(1) = 2
S(2) = 5
S(3) = 14
S(4) = 45
S(5) = 161

Note: This is the number for which there is NO coloring.

## Notes

- Many soloutions seem to be palindromes. It seems to me that they kind of have to be because of the constraints and the way the sums are constructed. If that's something one could prove then you would only need to find half solutions and mirror them.

- 

# Solutions

For any solution of K colors there are K! "recolorings": the same solution just with colors swapped.

Solutions for S(2):
abba
baab
Which are the same.

Solutions for S(3):
abbaccaccabba
bccbaacaabccb
caacbbabbcaac
cbbcaaaaacbbc



# Examples

The example directory contains files in the following format:
- 1 coloring per line
- Colors are denoted by unique characters
- Every character denotes the color of the digit at that position

Example:
```
abba
```
Defines a coloring for `{1,2,3,4}`, where color `a` is for `{1,4}` and color `b` is for `{2,3}`

## Filenames

Filenames are of the format `schur_k_n.txt`, where `k` is the number of colors used and `n` is the number of digits colored.

# Running

## Generate solutions

Run these with:
```
cargo run --bin shur -- --colors=3 --target=13 --algorithm=random_ban --attempts=100000
```
Parameters:
- `--colors=`: number of colors to use
- `--target=`: highest number to assign a color to
- `--algorithm`: The algorithm to use
- `--attempts`: (optional) limit for some algorithms.

Available algorithms: see ***Methods***

## Check solutions

```
cargo run --bin check -- --input=examples/shur_4_45.txt
```

# Visualization

Prerequisite: [graphviz](https://graphviz.org/download/) installed.

Use `create_dot` to create a dot file from a coloring, then use `dot` to create an image.
```bash
cargo run --bin create_dot -- --source=examples/shur_3_13.txt --destination=test.dot; and dot -Tsvg -otest.svg test.dot ; and open test.svg
```

# Methods

## Random color assignment

`algorithm=random`

Using default rand (which will divide numbers in more or less equal groups) running about a 100_000 attempts tends to find an answer for `(3,13)`

## Random color assigment with bans

`algorithm=random_ban`

Use default rand, but after assigning a color banning that color for higher numbers.
For example:
- if `1=a` then ban `a` for 2 (because 1+1=2)
- if `1=a`, `2=b`, `3=a` then ban `a` for `{4,6}`

This tends to find `(4,40)` in about 30_000 attempts. It cannot find `(4,44)` but instantly finds `(5,45)`

## Random Depth First Search

`algorithm=random_dfs`

Assign colors at random, keep any prefixes (prefix: sequence of color assignments starting at 1 up to N) that are valid and try to extend the longest prefix first.

Finds `(4,40)` in about two seconds.
Finds `(5,90)` in a minute, but at this point probably run with the `--release` profile.
Finds `(5,100)` in a many minutes, with a limit of 1000_000_000

## Search: Depth First

`algorithm=search_dsf`

Classic DFS. This should be worse than the random version I think. Since it never gets lucky and there is still a bound.

This finds 
- `(3,13)` with a limit of 300.
- `(4,44)` with a limit of 7_000_000 (<10 seconds)

## Search:: Breadth First