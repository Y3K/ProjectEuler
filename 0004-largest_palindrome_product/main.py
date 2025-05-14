#!/usr/bin/env python

def is_palindrome(number: int) -> bool:
    first = number
    second = 0

    while first > 0:
        digit = int(first % 10)
        second = second * 10 + digit
        first = int(first / 10)

    return number == second


def main():
    digits = 3
    limit = 10 ** digits
    largest = 0

    for i in reversed(range(1, limit)):
        for j in reversed(range(1, i)):
            product = i * j
            if product > largest and is_palindrome(product):
                largest = product

    print(largest)


if __name__ == "__main__":
    main()
