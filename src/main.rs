use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::thread;

const N_THREADS: usize = 8;
const LIMIT: usize = 100_000_000;

fn main() {
    // Start timer
    let start_time = Instant::now();

    let mut handles = vec![];

    let counter = Arc::new(Mutex::new(3));

    let num_primes = Arc::new(Mutex::new(0));

    // let mut largest_primes = Arc::new(Mutex::new(vec![0; 10]));

    // Create a vector to store the total of each thread
    let thread_totals = Arc::new(Mutex::new(vec![0; N_THREADS]));

    for index in 0..N_THREADS {
        let counter = Arc::clone(&counter);
        let thread_totals = Arc::clone(&thread_totals);
        let num_primes = Arc::clone(&num_primes);
        // let largest_primes = Arc::clone(&largest_primes);

        let handle = thread::spawn(move || {
            let mut total = 0;

            loop {
                let val;
                {
                    let mut num = counter.lock().unwrap();
                    val = *num;

                    *num += 2;
                }

                if val > LIMIT { break; }

                if is_prime(val) {
                    total += val;

                    let mut num_primes = num_primes.lock().unwrap();
                    *num_primes += 1;
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

    let time = start_time.elapsed();
    println!("Execution Time:  {:?}", time);

    let num_primes = num_primes.lock().unwrap();
    println!("Number of Primes Found: {}", num_primes);

    let mut total: usize = thread_totals.lock().unwrap().iter().sum();
    total += if LIMIT >= 2 { 2 } else { 0 };
    println!("Sum of Primes: {}", total);
}

// Determines whether a number is prime or not; uses memoization to prevent unnecessary calculations
fn is_prime(n: usize) -> bool {
    let upper = (n as f64).sqrt() as usize;

    // Base case so we can ignore all evens
    if n % 2 == 0 { return false; }

    for i in (3..=upper).step_by(2) {
        if n % i == 0 { return false; }
    }

    true
}
