#!/usr/bin/env python

def is_prime(number: int) -> bool:
    if number < 2:
        return False

    for i in range(2, number - 1):
        if number % i == 0:
            return False

    return True


def main():
    LIMIT = 10_001
    current = 0
    number = 1
    prime = None

    while current < LIMIT:
        if is_prime(number):
            prime = number
            current += 1

        number += 1

    print(prime)


if __name__ == "__main__":
    main()
