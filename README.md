# Rust PageRank Approximation Solution

## Introduction
This Rust program approximates the PageRank values for vertices in a directed graph based on a memoryless process of walking over vertices. The program reads the input graph from the provided ["data.txt"]([url](https://github.com/kthanasi-git/ds210-demo/blob/main/pagerank_data.txt)) file and outputs the labels of the five vertices with the highest approximate PageRank values, along with their respective values.

## Implementation
The program follows the specified rules for vertex selection during the random walks and simulates 100 random independent walks from each vertex. The approximation of PageRank for a vertex is determined by the fraction of random walks that terminated at that vertex (inspect main.rs for more)

## How To Run
Make sure you have Rust installed on your system.
Save the above code in a file named ``main.rs``
Open a terminal and navigate to the directory containing ``main.rs.``
Run the following commands:
```rust
cargo build --release
cargo run --release
```
## Output
The output of the program will provide you with the top 5 highest approximate PageRank vertices, and their associated PageRank score.
