# Even Fibonacci Numbers

Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:

```
1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
```

By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.


## Notes

I found this Fibonacci fn that uses an iterator instead of a vector:

```rust
fn fib_iter(limit: i32) -> impl Iterator<Item = i32> {
    let mut a = 0;
    let mut b = 1;

    std::iter::from_fn(move || {
        let next = a + b;
        if next > limit {
            None
        } else {
            a = b;
            b = next;
            Some(next)
        }
    })
}
```
