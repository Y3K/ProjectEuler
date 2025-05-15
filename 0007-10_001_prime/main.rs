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
    let limit: i64 = 10_001;
    let mut current: i64 = 0;
    let mut number: i64 = 1;
    let mut prime: i64 = 1;
    
    while current < limit {
        if is_prime(number) {
            prime = number;
            current += 1;
        }
        
        number += 1;
    }
    
    println!("{}", prime);
}
