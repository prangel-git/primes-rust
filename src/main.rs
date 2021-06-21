pub mod prime_sieve;

use crate::prime_sieve::PrimeSieve;

fn main() {
    let primes_from_iter = PrimeSieve::new()
        .take_while(|&p| p < 100)
        .collect::<Vec<_>>();

    println!("{:?}", primes_from_iter);
    println!("{}", primes_from_iter.len());

    let n = 10_001usize;
    let nth_prime = PrimeSieve::new().take(n).last().unwrap();
    println!("The {}-th prime is {}", n, nth_prime);

    let upper_bound = 2_000_000u64;
    let sum_primes: u64 = PrimeSieve::new().take_while(|&p| p < upper_bound).sum();
    println!("Sum of the first {} primes {}", upper_bound, sum_primes);
}
