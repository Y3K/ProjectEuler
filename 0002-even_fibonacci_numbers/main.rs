fn fib(limit: i32, mut last: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec!();
    let mut current: i32 = 1;
    
    loop {
        let new: i32 = current + last;        
        
        if new > limit {
            break;
        }
        
        result.push(new);
        last = current;
        current = new;
    }
    
    result
}

fn main() {
    let limit: i32 = 4_000_000;
    let result: i32 = fib(limit, 0)
        .into_iter()
        .filter(|n| n % 2 == 0)
        .sum();
        
    println!("{}", result);
}
