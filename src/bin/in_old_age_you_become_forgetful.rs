
fn main() {
    let target = 12345;
    let result = primes(target).sum::<usize>();

    println!("The sum of all primes up to {} is {}!", target, result);
}

fn primes(limit: usize) -> impl Iterator<Item = usize> {
    let mut is_prime = vec![true; limit + 1];

    is_prime[0] = false;
    is_prime[1] = false;

    for n in 2 ..= limit {
        if !is_prime[n] {
            continue
        }

        // Multiples of n are not prime
        for n in (n ..= limit).step_by(n).skip(1) {
            is_prime[n] = false;
        }
    }

    is_prime.into_iter()
        .enumerate()
        .filter_map(|(num, is_prime)| match is_prime {
            true => Some(num),
            false => None,
        })
}
