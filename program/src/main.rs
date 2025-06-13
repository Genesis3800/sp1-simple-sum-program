// sum/program/src/main.rs
#![no_main]
use sp1_zkvm::io::{read, commit};
use sp1_zkvm::entrypoint;

fn main() {
    // Read two private inputs
    let a: u32 = read();
    let b: u32 = read();
    
    // Compute the sum
    let sum = a.wrapping_add(b);
    
    // Commit the sum as a public output
    // This makes only the sum visible in the proof, while keeping a and b private
    commit(&sum);
}

// Define the entrypoint for the zkVM
entrypoint!(main);