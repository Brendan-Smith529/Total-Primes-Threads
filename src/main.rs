use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::thread;

const N_THREADS: usize = 8;
const LIMIT: usize = 100_000_000;

fn main() {
    // Start timer
    let start_time = Instant::now();

    /* Initialize info collection vars */

    // Stores all threads
    let mut handles = vec![];

    // Stores all primes
    let primes = Arc::new(Mutex::new(vec![]));

    // Shared counter
    let counter = Arc::new(Mutex::new(3));

    // Stores number of primes encountered
    let num_primes = Arc::new(Mutex::new(0));

    // Create a vector to store the total of each thread
    let thread_totals = Arc::new(Mutex::new(vec![0; N_THREADS]));

    for index in 0..N_THREADS {
        // Clones vars for each thread
        let counter = Arc::clone(&counter);
        let thread_totals = Arc::clone(&thread_totals);
        let num_primes = Arc::clone(&num_primes);
        let primes = Arc::clone(&primes);

        // Function that each thread runs
        let handle = thread::spawn(move || {
            let mut total = 0;

            loop {
                // Store current counter value in val
                let val;
                {
                    let mut num = counter.lock().unwrap();
                    val = *num;

                    *num += 2;
                }

                if val > LIMIT { break; }

                // If the numbers prime: add to num of primes and add to prime list
                if is_prime(val) {
                    total += val;

                    {
                        let mut num_primes = num_primes.lock().unwrap();
                        *num_primes += 1;
                    }

                    let mut primes = primes.lock().unwrap();
                    primes.push(val);
                }
            }

            // Adds total
            let mut thread_totals = thread_totals.lock().unwrap();
            thread_totals[index] = total;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Calculates execution time
    let time = start_time.elapsed();
    let mut text: String = format!("Execution Time:  {:?}\n", time);

    // States number of primes found
    let mut num_primes = num_primes.lock().unwrap();
    *num_primes += if LIMIT >= 2 { 1 } else { 0 };

    let num_primes_text: String = format!("Number of Primes Found: {}\n", num_primes);
    text.push_str(&num_primes_text);

    // Calculates total sum of primes found
    let mut total: usize = thread_totals.lock().unwrap().iter().sum();
    total += if LIMIT >= 2 { 2 } else { 0 };

    let total_text: String = format!("Sum of Primes: {}\n", total);
    text.push_str(&total_text);

    // Shows the ten largest primes
    let gen_text: String = format!("Ten Largest Primes: {{");
    text.push_str(&gen_text);

    let mut primes = primes.lock().unwrap();

    primes.push(2);

    // Get the 1000 most recent primes
    let prime_list: Vec<_> = primes
        .iter()
        .rev()
        .take(1000)
        .cloned()
        .collect();
    
    // Sort the primes
    let mut sorted_primes: Vec<_> = prime_list.clone();
    sorted_primes.sort_by(|a, b| b.cmp(a));

    // Take the largest 10 primes
    let result: Vec<_> = sorted_primes
        .into_iter()
        .take(10)
        .rev()
        .collect();

    let result_text: String = format!("{:?}}}\n", result);
    text.push_str(&result_text);

    // Prints result text to primes.txt
    let _ = fs::write("primes.txt", text);
}

// Determines whether a number is prime or not
fn is_prime(n: usize) -> bool {
    let upper = (n as f64).sqrt() as usize;

    for i in (3..=upper).step_by(2) {
        if n % i == 0 { return false; }
    }

    true
}
