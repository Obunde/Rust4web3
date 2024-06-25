fn main() {
    let start = 1;
    let end = 100; // Changed 'stop' to 'end' for consistency

    // Corrected the comparison to use 'end' instead of 'stop'
    if start > end {
        println!("start {} is greater than end {}, invalid input!", start, end);
        return; // Exit the program if input is invalid
    }
    
    // Call the sieve_of_eratosthenes function and store the result
    let primes = sieve_of_eratosthenes(start, end);
    
    // Print the prime numbers found between start and end
    println!("Prime numbers between {} and {} are: {:?}", start, end, primes);
}

fn sieve_of_eratosthenes(start: usize, end: usize) -> Vec<usize> {
    // If 'end' is less than 2, return an empty vector since there are no primes
    if end < 2 {
        return vec![];
    }

    // Initialize a vector with 'true' values indicating potential primes
    let mut is_prime = vec![true; end + 1];
    is_prime[0] = false; // 0 is not a prime number
    is_prime[1] = false; // 1 is not a prime number
    
    // Implement the Sieve of Eratosthenes algorithm
    for num in 2..=((end as f64).sqrt() as usize) {
        if is_prime[num] {
            for multiple in (num * num..=end).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }
    
    // Collect the prime numbers from the 'is_prime' vector
    is_prime.iter()
        .enumerate()
        .filter(|&(num, &prime)| prime && num >= start)
        .map(|(num, _)| num)
        .collect()
}
