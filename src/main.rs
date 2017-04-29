extern crate time;

mod alphametic;
mod permutation;
mod brute;

use std::collections::HashMap;
use std::env;
use std::ascii::AsciiExt;

static DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn insert_digits(word: &str, map: &HashMap<char, usize>) -> String {
    word.chars().map(|c| DIGITS[map[&c]]).collect()
}

fn print_perm(terms: &Vec<&str>, sum: &str, map: &HashMap<char, usize>) {

    println!("{} = {}",
             terms.join(" + "),
             sum);

    println!("{} = {}",
             terms.iter().map(|t| insert_digits(t, map)).collect::<Vec<_>>().join(" + "),
             insert_digits(sum, map));

}

fn main() {
    let start_time = time::precise_time_s();
    
    let mut terms: Vec<_> = env::args()
        .skip(1) //skip first arg. (the name of the program)
        .map(|s| s.to_ascii_uppercase()) //convert to uppercase strings
        .collect(); //make Vec<_>
    
    let sum = match terms.pop() {
        Some(s) => s,
        None => panic!("No arguments given!")
    };
    
    if terms.len() < 2 {
        panic!("Not enough arguments given!");
    }

    //let terms = vec![
    //    "AND",
    //    "A",
    //    "STRONG",
    //    "OFFENSE",
    //    "AS",
    //    "A",
    //    "GOOD"
    //];
    //let sum = "DEFENSE";


    let terms = terms.iter().map(AsRef::as_ref).collect();

    alphametic::solve(&terms, &sum, &|map| {

        print_perm(&terms, &sum, &map);
        println!();
    });
    
    println!("Completed in {:.4}s", time::precise_time_s() - start_time);
    
    
}
