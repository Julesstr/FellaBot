use std::collections::BTreeMap;
// use std::time::Instant;
use std::fs;

extern crate matchmaker;

use matchmaker::run_matchmaker;
use matchmaker::Solutions;


fn main() {
    let solutions: Solutions = run_matchmaker(1000000);

    let textfile: String = fs::read_to_string("../matchmaker/num.txt").expect("Failed to read file");
    let num_solutions: usize = textfile.trim().parse().expect("Not a valid usize");

    for n in 1..=num_solutions {
        println!("Solution number: {}", n);
        let result = solutions[n].clone();
        let teamone: BTreeMap<_, _> = result.1.into_iter().collect();
        let teamtwo: BTreeMap<_, _> = result.2.into_iter().collect();

        println!("Solution score: {}", result.0);
        println!("\r");

        println!("Team One:");
        for (key, value) in teamone {
            println!("Position {}: {}", key, value.1);
        }
        println!("\r");
        println!("Team Two:");
        for (key, value) in teamtwo {
            println!("Position {}: {}", key, value.1);
        }
        println!("\r");
        println!("\r");
    }

    fs::write("../matchmaker/num.txt", "1").expect("Failed to write to file");
}
