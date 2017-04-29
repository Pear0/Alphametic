
use std::collections::HashMap;

use brute;

fn all_chars(terms: &Vec<&str>, sum: &str) -> Vec<char> {
    let mut chars: Vec<char> = terms.iter().flat_map(|s| s.chars()).collect();
    for c in sum.chars() {
        chars.push(c);
    }
    chars.sort();
    chars.dedup();

    chars.into_iter().collect()
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

fn matcher(terms: &Vec<&str>, sum: &str, map: &HashMap<char, usize>) -> Option<HashMap<char, usize>> {

    let mut acc = 0;

    for term in terms {
        match str_to_num(&term.chars().collect(), &map) {
            Some(num) => acc += num,
            None => return None
        }
    }

    match str_to_num(&sum.chars().collect(), &map) {
        Some(num) => {

            if acc == num {
                return Some(map.clone());

            }else {
                return None
            }

        },
        None => return None
    }

}

pub fn solve<F>(terms: &Vec<&str>, sum: &str, callback: F) where F: Fn(HashMap<char, usize>) {
    let chars = all_chars(terms, sum);

    brute::parallel(10, 2, &chars,
                    |map| { matcher(terms, sum, map) },
                    callback);
}