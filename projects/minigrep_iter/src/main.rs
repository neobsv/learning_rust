use std::{env, process};
use minigrep_iter::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // The env::args function returns an iterator! Rather than collecting the iterator values into a vector and then passing a 
    // slice to Config::build, now we’re passing ownership of the iterator returned from env::args to Config::build directly.

    // Change the signature of Config::build as well, in lib.rs
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep_iter::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/* MINIGREP Vs MINIGREP_ITER

Which style do you prefer? Using Iterators or Using Loops?

Most Rust programmers prefer to use the iterator style. 
It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand. 
Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop. 
*/


/* Benchmarking: Loops vs. Iterators

To determine whether to use loops or iterators, you need to know which implementation is faster: 
the version of the search function with an explicit for loop or the version with iterators.

We ran a benchmark by loading the entire contents of The Adventures of Sherlock Holmes by Sir Arthur Conan Doyle into a String and looking for the word the in the contents. 
Here are the results of the benchmark on the version of search using the for loop and the version using iterators:

test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)

The iterator version was slightly faster! We won’t explain the benchmark code here, 
because the point is not to prove that the two versions are equivalent but to get a general sense of 
how these two implementations compare performance-wise.

RUST WIN: Zero Cost Abstractions!

The point is this: iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. 
Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.
The implementations of closures and iterators are such that runtime performance is not affected.

*/

// The following code is taken from an audio decoder. The decoding algorithm uses the linear prediction mathematical operation 
// to estimate future values based on a linear function of the previous samples.

#[allow(dead_code, unused_variables)]
fn audio_decoder_example() {

    // This code uses an iterator chain to do some math on three variables in scope: a buffer slice of data, an array of 12 coefficients, and an amount by which to shift data in qlp_shift.
    let buffer: &mut [i32] = &mut [11, 23, 3, 41, 5, 65, 7, 86, 97, 10, 81, 12];
    let coefficients: [i64; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let qlp_shift: i16 = 0;

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
                                    .zip(&buffer[i - 12..i])
                                    .map(|(&c, &s)| c * s as i64)
                                    .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
    
}

/* NOTE ABOUT OPTIMIZATIONS BY THE RUST COMPILER:

LOOP UNROLLING:
Here, we’re creating an iterator, using two adaptors, and then consuming the value. What assembly code would this Rust code compile to? 
It compiles down to the same assembly you’d write by hand. There’s no loop at all corresponding to the iteration over the values in coefficients: 
Rust knows that there are 12 iterations, so it “unrolls” the loop. Unrolling is an optimization that removes the overhead of the loop controlling code and instead generates 
repetitive code for each iteration of the loop.

REGISTERS:
All of the coefficients get stored in registers, which means accessing the values is very fast.

BOUNDS CHECK:
There are no bounds checks on the array access at runtime.

*/