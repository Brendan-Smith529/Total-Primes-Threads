# Total-Primes-Threads
Programming Assignment 1 for the class COP 4520 - Concepts of Parallel and Distributed Processing

Approach:\n
    \tMy approach is an optimization of the brute force approach to finding the sum of primes.
    \tThe reason I decided to use this approach is because it lends itself better to multi-threading
    \tthan that of a sieve (such as Sieve of Eratosthenes, Atkins, or Sundaram) since those algorithms
    \tare sequential and require the previous changes to an array/bitmap.

Informal Statement:\n
    \tThis approach is correct because it checks every number from 2..n for whether it is prime or not.
    \tIt's an efficient version as it removes all computation regarding even numbers and evenly splits the
    \twork up between eight threads.

Compilation Instructions:\n
    \tcargo build --release && ./target/release/temp
