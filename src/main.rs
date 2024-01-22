// use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::thread;

const N_THREADS: usize = 8;
const LIMIT: usize = 20;

fn main() {
    // Start timer
    let start_time = Instant::now();

    // Create shared counter an vector for all handles (start at 2 since 0 and 1 aren't prime)
    let counter = Arc::new(Mutex::new(2));
    let mut handles = vec![];

    // Create a vector to store the total of each thread
    let thread_totals = Arc::new(Mutex::new(vec![0; N_THREADS]));

    for index in 0..N_THREADS {
        let counter = Arc::clone(&counter);
        let thread_totals = Arc::clone(&thread_totals);

        let handle = thread::spawn(move || {
            let mut total = 0;

            loop {
                let val;
                {
                    let mut num = counter.lock().unwrap();
                    val = *num;

                    if val > LIMIT { break; }

                    *num += 1;
                }


                if is_prime(val) {
                    total += val;
                }
            }

            let mut thread_totals = thread_totals.lock().unwrap();
            thread_totals[index] = total;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total: usize = thread_totals.lock().unwrap().iter().sum();
    println!("Total: {}", total);

    let time = start_time.elapsed();
    println!("Time:  {:?}\n\n", time);
}

// Determines whether a number is prime or not; uses memoization to prevent unnecessary calculations
fn is_prime_std(n: usize) -> bool {
    let upper = (n as f64).sqrt() as usize;

    for i in 2..=upper {
        if n % i == 0 { return false; }
    }

    true
}
