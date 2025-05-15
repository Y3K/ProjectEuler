fn main() {
    let limit: i64 = 10;
    let sequence = 1..=limit;
    let sum_of_squares: i64 = sequence
        .clone()
        .map(|n| n * n)
        .sum();
    let mut square_of_sum: i64 = sequence.sum();
    square_of_sum *= square_of_sum;
    
    println!("{}", square_of_sum - sum_of_squares);
}
