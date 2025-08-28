fn main() {
    // iterations works on array by default
    // but it does not work on tuples,
    // this process is enabled by the IntoIteration Trait, we would get to that soon
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
            println!("prime: {}, i: {}", prime, i);
        }
    }
}
