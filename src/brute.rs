extern crate crossbeam;

use std::sync::mpsc;
use std::collections::HashMap;

use permutation::Permutation;

fn num_permutations(n: usize, k: usize) -> usize {
    let mut acc = 1;
    for i in (n-k+1)..n+1 {
        acc *= i
    }
    acc
}

pub fn parallel<M, T>(base: usize, thread_depth: usize, chars: &Vec<char>, matcher: &M, callback: &Fn(T))
    where M: Fn(&HashMap<char, usize>) -> Option<T> + Send + Sync, T: Send {

    let digits: Vec<usize> = (0..base).collect();
    let thread_count = num_permutations(base, thread_depth);

    println!("{:?} P {:?} = {:?}", base, thread_depth, thread_count);

    crossbeam::scope(|scope| {
        let (tx, rx) = mpsc::channel::<Option<T>>();

        for i in 0..thread_count {
            let mut prefix: Vec<usize> = Vec::with_capacity(thread_depth);
            let mut numbers = digits.clone();
            let tx = tx.clone();

            scope.spawn(move || {
                let mut i = i;
                for _ in 0..thread_depth {
                    let len = numbers.len();
                    prefix.push(numbers.remove(i % len));
                    i /= len;
                }
                let prefix = prefix;

                let numbers = numbers;

                for mut perm in Permutation::new(&numbers, chars.len() - thread_depth) {
                    for j in &prefix {
                        perm.push(*j);
                    }

                    //println!("{:?}", perm);

                    let mut map: HashMap<char, usize> = HashMap::with_capacity(chars.len());
                    for i in 0..chars.len() {
                        map.insert(chars[i], perm[i]);
                    }

                    match matcher(&map) {
                        Some(result) => {
                            tx.send(Some(result)).ok();
                        },
                        None => continue
                    }

                }

                tx.send(None).ok();

            });
        }

        let mut threads_left = thread_count;
        while threads_left > 0 {
            match rx.recv().unwrap() {
                Some(perm) => { callback(perm) },
                None => threads_left -= 1
            }
        }
    });

}