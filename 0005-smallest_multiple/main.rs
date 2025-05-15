fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn main() {
    let limit: i64 = 10;
    let mut sequence: Vec<i64> = (1..limit).collect();
    let mut result: i64 = sequence[0];
    
    for number in &mut sequence[1..] {
        result = lcm(result, *number);
    }
    
    println!("{}", result);
}
