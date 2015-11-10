extern crate time;

use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use std::env;
use std::ascii::AsciiExt;

struct Permutation {
    elements: Vec<usize>,
    swappings: Vec<usize>,
    valid: bool
}

impl Iterator for Permutation {
    type Item = Vec<usize>;
    
    fn next(&mut self) -> Option<Vec<usize>> {
        if !self.valid {
            return None
        }
        
        let res = self.elements
            .iter()
            .take(self.swappings.len())
            .map(|n| n.clone())
            .collect::<Vec<_>>();
        
        let mut i: isize = self.swappings.len() as isize - 1;
        
        while i >= 0 && self.swappings[i as usize] == self.elements.len() - 1 {
            self.elements.swap(i as usize, self.swappings[i as usize]);
            self.swappings[i as usize] = i as usize;
            i -= 1;
        }
        
        
        if i < 0 {
            self.valid = false
        }else {
            let prev = self.swappings[i as usize];
            self.elements.swap(i as usize, prev);
            let next = prev + 1;
            self.swappings[i as usize] = next;
            self.elements.swap(i as usize, next);
        }
        
        Some(res)
    }
    
}

fn permutation(elements: &Vec<usize>, num: usize) -> Permutation {
    Permutation {
        elements: elements.clone(), 
        swappings: (0..num).collect::<Vec<_>>(),
        valid: true
    }
}

fn str_to_num(word: &Vec<char>, map: &HashMap<char, usize>) -> Option<usize> {
    let mut acc = 0;
    let mut mul = 1;
    
    for i in (0..word.len()).rev() {
        
        let digit = map.get(&word[i]).unwrap().clone();
        if i == 0 && digit == 0 {
            return None;
        }
        
        acc += mul * digit;
        mul *= 10;
        
    }
    
    Some(acc)
}

fn print_perm_map(terms: &Vec<Vec<char>>, sum: &Vec<char>, map: &HashMap<char, usize>) {
    let numbers = "0123456789".chars().collect::<Vec<_>>();
    
    println!("{} = {}", 
        terms.iter().map(|t| t.iter().cloned().collect::<String>()).collect::<Vec<_>>().join(" + "), 
        sum.iter().cloned().collect::<String>());
    
    println!("{} = {}", 
        terms.iter().map(|t| t.iter().map(|k| numbers[map.get(k).unwrap().clone()].clone()).collect::<String>()).collect::<Vec<_>>().join(" + "), 
        sum.iter().map(|k| numbers[map.get(k).unwrap().clone()].clone()).collect::<String>());
    
}

fn print_perm(terms: &Vec<Vec<char>>, sum: &Vec<char>, chars: &Vec<char>, perm: &Vec<usize>) {
    let mut map: HashMap<char, usize> = HashMap::with_capacity(chars.len());
    for i in 0..chars.len() {
        map.insert(chars[i], perm[i]);
    }
    print_perm_map(terms, sum, &map);
}

fn main() {
    let start_time = time::precise_time_s();
    
    let mut terms: Vec<_> = env::args()
        .skip(1) //skip first arg. (the name of the program)
        .map(|s| s.to_ascii_uppercase().chars().collect::<String>()) //convert to uppercase strings
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
    
    let terms = terms.iter().cloned().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let sum = sum.clone().chars().collect::<Vec<_>>();
    
    let mut chars: Vec<char> = terms.iter().flat_map(|s| s.clone().into_iter()).collect();
    for c in &sum {
        chars.push(c.clone());
    }
    chars.sort();
    chars.dedup();
    let chars: Vec<char> = chars.into_iter().collect();
    
    let (tx, rx) = mpsc::channel();
    
    for j in 0..10 { for k in (0..10).filter(|x| x.clone() != j) {
        let (tx, terms, sum, chars) = (tx.clone(), terms.clone(), sum.clone(), chars.clone());
        
        let mut numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        numbers.remove(j);
        numbers.remove(if k > j {k - 1} else {k});
        
        thread::spawn(move || {
            'outer: for perm in permutation(&numbers, chars.len() - 2) {
                let mut perm = perm;
                perm.push(j);
                perm.push(k);
                
                let mut map: HashMap<char, usize> = HashMap::with_capacity(chars.len());
                for i in 0..chars.len() {
                    map.insert(chars[i], perm[i]);
                }
                
                let mut left = 0;
                
                for i in 0..terms.len() {
                    match str_to_num(&terms[i], &map) {
                        Some(num) => left += num,
                        None => continue 'outer
                    }
                }
                
                match str_to_num(&sum, &map) {
                    Some(num) => {
                        
                        if left == num {
                            tx.send(Some(perm.clone())).ok();
                            
                        }
                        
                    },
                    None => continue 'outer
                }
                
            }
            
            tx.send(None).ok();
        });
    }}
    
    let mut i = 0;
    while i < 90 {
        match rx.recv().unwrap() {
            Some(perm) => {
                print_perm(&terms, &sum, &chars, &perm);
                println!("");
            },
            None => i += 1
        }
    }
    
    println!("Completed in {:.4}s", time::precise_time_s() - start_time);
    
    
}
