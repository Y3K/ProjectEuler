fn is_prime(number: i64) -> bool {
    if number < 2 {
        return false;
    }
    
    for i in 2..(number - 1) {
        if number % i == 0 {
            return false;
        }
    }
    
    return true;
}

fn main() {
    let mut number: i64 = 600_851_475_143;
    let limit: i64 = (number as f64).sqrt() as i64;
    
    for prime in 2..limit {
        if !is_prime(prime) {
            continue
        }
        
        if number % prime == 0 {
            number /= prime;
        }
        
        if number == 1 {
            println!("{}", prime);
            break;
        }
    }
}
