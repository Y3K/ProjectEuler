#!/usr/bin/env python

def main():
    limit = 100
    sequence = list(range(1, limit + 1))
    sum_of_squares = sum(n * n for n in sequence)
    square_of_sum = sum(sequence)
    square_of_sum *= square_of_sum
    print(square_of_sum - sum_of_squares)


if __name__ == "__main__":
    main()
